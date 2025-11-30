use crate::*;

pub struct DigitalInputMqtt {
    state: bool,
    topic: String,
}

impl DigitalInput for DigitalInputMqtt {
    fn get_state(&self) -> bool {
        self.state
    }
}

pub struct DigitalOutputMqtt {
    state: bool,
    topic: String,
}

impl DigitalOutput for DigitalOutputMqtt {
    fn set_output(&mut self, value: bool) {
        self.state = value;
    }

    fn set_true(&mut self) {
        self.state = true;
    }

    fn set_false(&mut self) {
        self.state = false;
    }
}

impl DigitalInput for DigitalOutputMqtt {
    fn get_state(&self) -> bool {
        self.state
    }
}

#[cfg(test)]
mod test {
    use crate::{DigitalInput, DigitalOutput};

    #[test]
    fn digital_input() {
        let input = super::DigitalInputMqtt {
            state: true,
            topic: "".to_string(),
        };
        assert!(input.get_state());
        let input = super::DigitalInputMqtt {
            state: false,
            topic: "".to_string(),
        };
        assert!(!input.get_state());
    }

    #[test]
    fn digital_output() {
        let mut output = super::DigitalOutputMqtt {
            state: false,
            topic: "".to_string(),
        };
        assert!(!output.get_state());
        output.set_true();
        assert!(output.get_state());
        output.set_false();
        assert!(!output.get_state());
        output.set_output(true);
        assert!(output.get_state());
        output.set_output(false);
        assert!(!output.get_state());
    }
}
