use quad_snd::{AudioContext, PlaySoundParams};

pub struct Sound {
    ctx: AudioContext,
    sound_content: quad_snd::Sound,
}

impl Sound {
    pub fn new(bytes: &[u8]) -> Self {
        let ctx = AudioContext::new();
        let sound_content = quad_snd::Sound::load(&ctx, bytes);
        Self { ctx, sound_content }
    }
    // ffmpeg -i input.mp3 -c:a libvorbis -qscale:a 0 output.ogg
    pub fn play(&self, flag: &mut bool, looped: bool, volume: f32) {
        if flag == &true {
            let params = PlaySoundParams { looped, volume };
            self.sound_content.play(&self.ctx, params);
            *flag = false;
        }
    }
}
