use error::Error;
use failure::Fail;
//use num_rational::Rational64;
use socool_ast::{NormalForm, PointOp};
use weresocool::{
    generation::{
        filename_to_render,
        parsed_to_render::r_to_f64,
        renderable::{nf_to_vec_renderable, RenderOp},
        RenderReturn, RenderType,
    },
    instrument::{Oscillator, StereoWaveform},
    settings::default_settings,
};

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            for cause in Fail::iter_causes(&e.unwrap_err()) {
                println!("Failure caused by: {}", cause);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderVoice {
    pub sample_index: usize,
    pub op_index: usize,
    pub ops: Vec<RenderOp>,
    pub oscillator: Oscillator,
}

impl RenderVoice {
    pub fn init(ops: &Vec<RenderOp>) -> RenderVoice {
        RenderVoice {
            sample_index: 0,
            op_index: 0,
            ops: ops.to_vec(),
            oscillator: Oscillator::init(&default_settings()),
        }
    }

    pub fn get_batch(
        &mut self,
        samples_left_in_batch: usize,
        result: Option<Vec<RenderOp>>,
    ) -> Vec<RenderOp> {
        let mut result: Vec<RenderOp> = match result {
            Some(result) => result.to_vec(),
            None => vec![],
        };

        let current_op = &self.ops[self.op_index];
        dbg!(self.sample_index, current_op.samples);

        if (current_op.samples - self.sample_index) > samples_left_in_batch {
            result.push(RenderOp {
                samples: samples_left_in_batch,
                index: self.sample_index,
                ..*current_op
            });
            self.sample_index += samples_left_in_batch;
        } else {
            let n_samples = current_op.samples - self.sample_index;
            result.push(RenderOp {
                samples: n_samples,
                index: self.sample_index,
                ..*current_op
            });
            self.op_index += 1;
            self.sample_index = 0;

            return self.get_batch(samples_left_in_batch - n_samples, Some(result));
        }

        result
    }

    pub fn render_batch(&mut self, n_samples: usize) -> StereoWaveform {
        let batch = self.get_batch(n_samples, None);
        unimplemented!()
    }
}

pub fn renderables_to_render_voices(renderables: Vec<Vec<RenderOp>>) -> Vec<RenderVoice> {
    renderables
        .iter()
        .map(|voice| RenderVoice::init(voice))
        .collect::<Vec<RenderVoice>>()
}

#[test]
fn test_get_batch() {
    let mut nf = NormalForm::init();
    let filename = "songs/test/render_op_get_batch.socool".to_string();
    let (nf, basis, table) =
        match filename_to_render(&filename, RenderType::NfBasisAndTable).unwrap() {
            RenderReturn::NfBasisAndTable(nf, basis, table) => (nf, basis, table),
            _ => {
                panic!();
            }
        };
    let renderables = nf_to_vec_renderable(&nf, &table, &basis);
    let voices = renderables_to_render_voices(renderables);
    let mut voice = voices[0].clone();
    let batch = voice.get_batch(44_000, None);
    let batch = voice.get_batch(200, None);
    assert_eq!(batch[0].samples, 100);
    assert_eq!(batch[0].index, 44_000);
    assert_eq!(batch[0].f, 220.0);
    assert_eq!(batch[1].samples, 100);
    assert_eq!(batch[1].index, 0);
    assert_eq!(batch[1].f, 247.5);

    dbg!(batch);
}

#[allow(unused_variables)]
fn run() -> Result<(), Error> {
    let (nf, basis, table) =
        match filename_to_render("songs/test/live.socool", RenderType::NfBasisAndTable)? {
            RenderReturn::NfBasisAndTable(nf, basis, table) => (nf, basis, table),
            _ => panic!("Error. Unable to generate NormalForm"),
        };
    let renderables = nf_to_vec_renderable(&nf, &table, &basis);
    let voices = renderables_to_render_voices(renderables);
    //let basis_f = r_to_f64(basis.f);

    Ok(())
}
