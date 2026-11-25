//! The bytes of a blob: the proved actions of one intent. A count or a length is 8 big-endian bytes, a flag or a
//! deletion criterion 1 byte, a field element 32 big-endian bytes, and a point its two coordinates, with the point at
//! infinity as (0, 0).

use anoma_arm::action::{Action, ResourceInstance};
use anoma_arm::app_data::{AppData, Blob, DeletionCriterion};
use anoma_arm::field::{coordinates, from_bytes as field_from_bytes, to_bytes as field_to_bytes};
use anoma_arm::{Fr, Point};
use anoma_prover::action::{LogicProof, ProvedAction};
use anoma_prover::circuit_prover::{Proof, VerificationKey};
use ark_ec::AffineRepr;

pub fn to_bytes(actions: &[ProvedAction]) -> Vec<u8> {
    let mut writer = Writer(Vec::new());
    writer.length(actions.len());
    for proved in actions {
        writer.list(&proved.action.resources, Writer::resource);
        writer.point(proved.delta);
        writer.proof(&proved.compliance_proof);
        writer.list(&proved.logic_proofs, Writer::logic_proof);
    }
    writer.0
}

/// The actions of the bytes, if the bytes are exactly the encoding of actions: each field element is below the
/// modulus, each flag and deletion criterion is 0 or 1, and each delta is a point of the curve.
pub fn from_bytes(bytes: &[u8]) -> Option<Vec<ProvedAction>> {
    let mut reader = Reader(bytes);
    let actions = reader.list(|reader| {
        Some(ProvedAction {
            action: Action {
                resources: reader.list(Reader::resource)?,
            },
            delta: reader.point()?,
            compliance_proof: reader.proof()?,
            logic_proofs: reader.list(Reader::logic_proof)?,
        })
    })?;
    reader.0.is_empty().then_some(actions)
}

struct Writer(Vec<u8>);

impl Writer {
    fn resource(&mut self, resource: &ResourceInstance) {
        self.0.push(u8::from(resource.is_consumed));
        self.field(resource.tag);
        self.field(resource.logic_ref);
        self.field(resource.commitment_tree_root);
        self.app_data(&resource.app_data);
    }

    fn app_data(&mut self, app_data: &AppData) {
        for payload in [
            &app_data.resource_payload,
            &app_data.discovery_payload,
            &app_data.external_payload,
            &app_data.application_payload,
        ] {
            self.list(payload, Self::blob);
        }
    }

    fn blob(&mut self, blob: &Blob) {
        self.0.push(blob.deletion_criterion as u8);
        self.length(blob.bytes.len());
        self.0.extend_from_slice(&blob.bytes);
    }

    fn logic_proof(&mut self, logic_proof: &LogicProof) {
        self.fields(&logic_proof.verification_key.fields);
        self.field(logic_proof.verification_key.hash);
        self.proof(&logic_proof.proof);
    }

    fn proof(&mut self, proof: &Proof) {
        self.fields(&proof.fields);
        self.fields(&proof.public_inputs);
    }

    fn point(&mut self, point: Point) {
        let (x, y) = coordinates(point);
        self.field(x);
        self.field(y);
    }

    fn fields(&mut self, fields: &[Fr]) {
        self.list(fields, |writer, &field| writer.field(field));
    }

    fn list<T>(&mut self, items: &[T], mut item: impl FnMut(&mut Self, &T)) {
        self.length(items.len());
        for value in items {
            item(self, value);
        }
    }

    fn field(&mut self, field: Fr) {
        self.0.extend_from_slice(&field_to_bytes(field));
    }

    /// A `usize` has at most 64 bits on every target of Rust, so the cast keeps the value.
    fn length(&mut self, length: usize) {
        self.0.extend_from_slice(&(length as u64).to_be_bytes());
    }
}

struct Reader<'a>(&'a [u8]);

impl Reader<'_> {
    fn resource(&mut self) -> Option<ResourceInstance> {
        let is_consumed = match self.take::<1>()? {
            [0] => false,
            [1] => true,
            _ => return None,
        };
        Some(ResourceInstance {
            is_consumed,
            tag: self.field()?,
            logic_ref: self.field()?,
            commitment_tree_root: self.field()?,
            app_data: self.app_data()?,
        })
    }

    fn app_data(&mut self) -> Option<AppData> {
        Some(AppData {
            resource_payload: self.list(Self::blob)?,
            discovery_payload: self.list(Self::blob)?,
            external_payload: self.list(Self::blob)?,
            application_payload: self.list(Self::blob)?,
        })
    }

    fn blob(&mut self) -> Option<Blob> {
        let deletion_criterion = match self.take::<1>()? {
            [0] => DeletionCriterion::Immediately,
            [1] => DeletionCriterion::Never,
            _ => return None,
        };
        let length = self.length()?;
        let (bytes, rest) = self.0.split_at_checked(length)?;
        self.0 = rest;
        Some(Blob {
            deletion_criterion,
            bytes: bytes.to_vec(),
        })
    }

    fn logic_proof(&mut self) -> Option<LogicProof> {
        Some(LogicProof {
            verification_key: VerificationKey {
                fields: self.list(Self::field)?,
                hash: self.field()?,
            },
            proof: self.proof()?,
        })
    }

    fn proof(&mut self) -> Option<Proof> {
        Some(Proof {
            fields: self.list(Self::field)?,
            public_inputs: self.list(Self::field)?,
        })
    }

    fn point(&mut self) -> Option<Point> {
        let (x, y) = (self.field()?, self.field()?);
        if (x, y) == coordinates(Point::zero()) {
            return Some(Point::zero());
        }
        let point = Point::new_unchecked(x, y);
        (point.is_on_curve() && point.is_in_correct_subgroup_assuming_on_curve()).then_some(point)
    }

    /// A count, then that many items. It reserves no space for the count, which comes from the blob.
    fn list<T>(&mut self, mut item: impl FnMut(&mut Self) -> Option<T>) -> Option<Vec<T>> {
        let count = self.length()?;
        let mut items = Vec::new();
        for _ in 0..count {
            items.push(item(self)?);
        }
        Some(items)
    }

    fn field(&mut self) -> Option<Fr> {
        field_from_bytes(self.take()?).ok()
    }

    fn length(&mut self) -> Option<usize> {
        usize::try_from(u64::from_be_bytes(self.take()?)).ok()
    }

    fn take<const N: usize>(&mut self) -> Option<[u8; N]> {
        let (chunk, rest) = self.0.split_first_chunk::<N>()?;
        self.0 = rest;
        Some(*chunk)
    }
}
