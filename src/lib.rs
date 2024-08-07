#[derive(Debug)]
pub enum Event<T = i32> where T: TryFrom<i32> {
//// Controller unplugged.
    Disconnect,
    /// Exit / Main / Home / Mode
    Exit(bool),
    /// A / 1 / 4 / Circle.  Action A (Primary action).
    ActionA(bool),
    /// B / 2 / 3 / Cross.  Action B (Secondary action).
    ActionB(bool),
    /// C.  Action C (Tertiary action).
    ActionC(bool),
    /// Y / X / Square.  Action H (Horizontal action).
    ActionH(bool),
    /// X / Y / Triangle.  Action V (Vertical action).
    ActionV(bool),
    /// Z (in 6-button layout).  Action D.
    ActionD(bool),
    /// Left Menu / Back / Select / Minus / Stop
    MenuL(bool),
    /// Right Menu / Forward / Start / Plus / Play
    MenuR(bool),
    /// Thumb Push Button On Main / Left Joystick
    Joy(bool),
    /// Thumb Push Button On Camera / Right Joystick
    Cam(bool),
    /// Left shoulder button (near button if no trigger)
    BumperL(bool),
    /// Right shoulder button (near button if no trigger)
    BumperR(bool),
    /// Left Bumper Trigger (far button if no trigger)
    TriggerL(T),
    /// Right Bumper Trigger (far button if no trigger)
    TriggerR(T),
    /// D-Pad Up
    Up(bool),
    /// D-Pad Down
    Down(bool),
    /// D-Pad Left
    Left(bool),
    /// D-Pad Right
    Right(bool),
    /// POV/Main Hat Left
    PovUp(bool),
    /// POV/Main Hat Down
    PovDown(bool),
    /// POV/Main Hat Left
    PovLeft(bool),
    /// POV/Main Hat Right
    PovRight(bool),
    /// Extra Hat Up
    HatUp(bool),
    /// Extra Hat Down
    HatDown(bool),
    /// Extra Hat Left
    HatLeft(bool),
    /// Extra Hat Right
    HatRight(bool),
    /// Trim Hat Up
    TrimUp(bool),
    /// Trim Hat Down
    TrimDown(bool),
    /// Trim Hat Left
    TrimLeft(bool),
    /// Trim Hat Right
    TrimRight(bool),
    /// Mic Hat Up
    MicUp(bool),
    /// Mic Hat Down
    MicDown(bool),
    /// Mic Hat Left
    MicLeft(bool),
    /// Mic Hat Right
    MicRight(bool),
    /// Main stick horizontal axis (A / D)
    JoyX(T),
    /// Main stick vertical / depth axis (W / S)
    JoyY(T),
    /// Main stick rotation / yaw axis
    JoyZ(T),
    /// Secondary stick X axis (Mouse X Position)
    CamX(T),
    /// Secondary stick Y axis (Mouse Y Position)
    CamY(T),
    /// Secondary stick Z axis
    CamZ(T),
    /// Slew Control
    Slew(T),
    /// Stationary throttle
    Throttle(T),
    /// Left stationary throttle
    ThrottleL(T),
    /// Right stationary throttle
    ThrottleR(T),
    /// Volume axis
    Volume(T),
    /// Steering wheel
    Wheel(T),
    /// Ship rudder
    Rudder(T),
    /// Gas Pedal
    Gas(T),
    /// Brake Pedal
    Brake(T),
    /// Mic Hat Push Button
    MicPush(bool),
    /// Flightstick trigger button on the back.
    Trigger(bool),
    /// Flightstick Side Bumper Button
    Bumper(bool),
    /// Flightstick Top Middle Action Button
    ActionM(bool),
    /// Flightstick Top Left Action Button
    ActionL(bool),
    /// Flightstick Top Right Action Button
    ActionR(bool),
    /// Pinky Button
    Pinky(bool),
    /// Pinky three-way switch Forward.
    PinkyForward(bool),
    /// Pinky three-way switch Backward.
    PinkyBackward(bool),
    /// Flaps three-way switch Forward.
    /// - `true` - Forward (Up)
    /// - `false` - Neutral (Maneuver)
    FlapsUp(bool),
    /// Flaps three-way switch Backward.
    /// - `true` - Backward (Down)
    /// - `false` - Neutral (Maneuver)
    FlapsDown(bool),
    /// Boat three-way switch Forward.
    BoatForward(bool),
    /// Boat three-way switch Backward.
    BoatBackward(bool),
    /// Autopilot three-way switch Forward.
    /// - `true` - Forward (Path)
    /// - `false` - Neutral (Altitude / Heading)
    AutopilotPath(bool),
    /// Autopilot three-way switch Backward.
    /// - `true` - Backward (Alt)
    /// - `false` - Neutral (Altitude / Heading)
    AutopilotAlt(bool),
    /// Left Engine Operate three-way switch Backward.
    /// - `true` - Backward (Motor)
    /// - `false` - Neutral (Normal)
    EngineMotorL(bool),
    /// Right Engine Operate three-way switch Backward.
    /// - `true` - Backward (Motor)
    /// - `false` - Neutral (Normal)
    EngineMotorR(bool),
    /// Engine Fuel Flow Left two-way switch
    /// - `true` - Normal
    /// - `false` - Override
    EngineFuelFlowL(bool),
    /// Engine Fuel Flow Right two-way switch
    /// - `true` - Normal
    /// - `false` - Override
    EngineFuelFlowR(bool),
    /// Left Engine Operate three-way switch Forward.
    /// - `true` - Forward (Ignition)
    /// - `false` - Neutral (Normal)
    EngineIgnitionL(bool),
    /// Right Engine Operate three-way switch Forward.
    /// - `true` - Forward (Ignition)
    /// - `false` - Neutral (Normal)
    EngineIgnitionR(bool),
    /// Speedbrake three-way switch Backward.
    SpeedbrakeBackward(bool),
    /// Speedbrake three-way switch Forward.
    SpeedbrakeForward(bool),
    /// China hat three-way switch Backward.
    ChinaBackward(bool),
    /// China hat three-way switch Forward.
    ChinaForward(bool),
    /// APU (Auxiliary Power Unit) two-way switch
    /// - `true` - Start
    /// - `false` - Off
    Apu(bool),
    /// Radar Altimeter two-way switch (Altitude measurements)
    /// - `true` - Normal
    /// - `false` - Disabled
    RadarAltimeter(bool),
    /// Landing Gear Horn Silence Button
    LandingGearSilence(bool),
    /// EAC (Enhanced Attitude Control - Autopilot) two-way switch
    /// - `true` - Arm
    /// - `false` - Off
    Eac(bool),
    /// Autopilot Toggle Button
    AutopilotToggle(bool),
    /// Throttle button (Left)
    ThrottleButton(bool),
    /// Mouse delta position horizontal
    MouseX(T),
    /// Mouse delta position vertical
    MouseY(T),
    /// Mouse primary button
    Mouse(bool),
    /// Numbered or unlabeled programmable action buttons (If unlabelled,
    /// prefer numbering from left to right, upper to lower)
    Number(i8, bool),
    /// Back left grip button (upper if there are two)
    PaddleLeft(bool),
    /// Back right grip button (upper if there are two)
    PaddleRight(bool),
    /// Left Pinky Button / Back lower right grip button
    PinkyLeft(bool),
    /// Right Pinky Button / Back lower left grip button
    PinkyRight(bool),
    /// Context Menu Button on a mouse (Right Click)
    Context(bool),
    /// DPI Button on a mouse
    Dpi(bool),
    /// Scroll Wheel X on a mouse
    ScrollX(T),
    /// Scroll Wheel Y on a mouse
    ScrollY(T),
    /// Scroll Button on a mouse
    Scroll(bool),
    /// Horizontal axis under the action buttons
    ActionWheelX(T),
    /// Vertical axis under the action buttons
    ActionWheelY(T),
}

use input_linux_sys::*;

impl <T> Event<T> where T: TryFrom<i32> {
    pub fn from_input_event(input_event: &input_event, state: &mut u8) -> Option<Self> {
        match input_event.type_ as _ {
            EV_ABS => Self::from_abs_event(input_event, state),
            EV_CNT => todo!(),
            EV_FF => None, //ignore force feedback events
            EV_FF_STATUS => todo!(),
            EV_KEY => Self::from_key_event(input_event),
            EV_LED => todo!(),
            EV_MAX => todo!(),
            EV_MSC => { //ignore misc/scan events
                if input_event.code != 4 {
                    panic!()
                }
                None
            }
            EV_PWR => todo!(),
            EV_REL => Event::from_rel_event(input_event),
            EV_REP => todo!(),
            EV_SND => todo!(),
            EV_SW => todo!(),
            EV_SYN => None, //ignore syn input events
            EV_UINPUT => todo!(),
            _unknown => panic!("unknown input type"),
        }
    }

    fn from_key_event(input_event: &input_event) -> Option<Self> {
        let key = input_event.code;
        let val = input_event.value.try_into().ok()?;
        let pushed = input_event.value != 0;

        Some(match key {
            0x08B /* KEY_MENU */ => Event::Context(pushed),

            0x09E /* KEY_BACK */ => Event::PaddleLeft(pushed),
            0x09F /* KEY_FORWARD */ => Event::PaddleRight(pushed),

            0x120 /* BTN_TRIGGER */ => Event::Trigger(pushed),
            0x121 /* BTN_THUMB */ => Event::ActionM(pushed),
            0x122 /* BTN_THUMB2 */ => Event::Bumper(pushed),
            0x123 /* BTN_TOP */ => Event::ActionR(pushed),
            0x124 /* BTN_TOP2 */ => Event::ActionL(pushed),
            0x125 /* BTN_PINKIE */ => Event::Pinky(pushed),
            0x126 /* BTN_BASE1 */ => Event::Number(1, pushed),
            0x127 /* BTN_BASE2 */ => Event::Number(2, pushed),
            0x128 /* BTN_BASE3 */ => Event::Number(3, pushed),
            0x129 /* BTN_BASE4 */ => Event::Number(4, pushed),
            0x12A /* BTN_BASE5 */ => Event::Number(5, pushed),
            0x12B /* BTN_BASE6 */ => Event::Number(6, pushed),
            0x12C /* BTN_BASE7 */ => Event::Number(7, pushed),
            0x12D /* BTN_BASE8 */ => Event::Number(8, pushed),
            0x12E /* BTN_BASE9 */ => Event::Number(9, pushed),
            0x12F /* BTN_BASE10 */ => Event::Number(10, pushed),

            0x130 /* BTN_A / BTN_SOUTH */ => Event::ActionA(pushed),
            0x131 /* BTN_B / BTN_EAST */ => Event::ActionB(pushed),
            0x132 /* BTN_C */ => Event::ActionC(pushed),
            0x133 /* BTN_X / BTN_NORTH */ => Event::ActionV(pushed),
            0x134 /* BTN_Y / BTN_WEST */ => Event::ActionH(pushed),
            0x135 /* BTN_Z */ => Event::ActionD(pushed),
            0x136 /* BTN_TL */ => Event::BumperL(pushed),
            0x137 /* BTN_TR */ => Event::BumperR(pushed),
            0x138 /* BTN_TL2 */ => Event::TriggerL(val),
            0x139 /* BTN_TR2 */ => Event::TriggerR(val),
            0x13A /* BTN_SELECT */ => Event::MenuL(pushed),
            0x13B /* BTN_START */ => Event::MenuR(pushed),
            0x13C /* BTN_MODE */ => Event::Exit(pushed),
            0x13D /* BTN_THUMBL */ => Event::Joy(pushed),
            0x13E /* BTN_THUMBR */ => Event::Cam(pushed),
            0x13F /* BTN_PINKYR */ => Event::PinkyRight(pushed),
            0x140 /* BTN_PINKYL */ => Event::PinkyLeft(pushed),

            0x220 /* BTN_DPAD_UP */ => Event::Up(pushed),
            0x221 /* BTN_DPAD_DOWN */ => Event::Down(pushed),
            0x222 /* BTN_DPAD_LEFT */ => Event::Left(pushed),
            0x223 /* BTN_DPAD_RIGHT */ => Event::Right(pushed),

            0x2C0 /* BTN_TRIGGER_HAPPY1 */ => Event::Number(11, pushed),
            0x2C1 /* BTN_TRIGGER_HAPPY2 */ => Event::Number(12, pushed),
            0x2C2 /* BTN_TRIGGER_HAPPY3 */ => Event::Number(13, pushed),
            0x2C3 /* BTN_TRIGGER_HAPPY4 */ => Event::Number(14, pushed),
            0x2C4 /* BTN_TRIGGER_HAPPY5 */ => Event::Number(15, pushed),
            0x2C5 /* BTN_TRIGGER_HAPPY6 */ => Event::Number(16, pushed),
            0x2C6 /* BTN_TRIGGER_HAPPY7 */ => Event::Number(17, pushed),
            0x2C7 /* BTN_TRIGGER_HAPPY8 */ => Event::Number(18, pushed),
            0x2C8 /* BTN_TRIGGER_HAPPY9 */ => Event::Number(19, pushed),
            0x2C9 /* BTN_TRIGGER_HAPPY10 */ => Event::Number(20, pushed),
            0x2CA /* BTN_TRIGGER_HAPPY11 */ => Event::Number(21, pushed),
            0x2CB /* BTN_TRIGGER_HAPPY12 */ => Event::Number(22, pushed),
            0x2CC /* BTN_TRIGGER_HAPPY13 */ => Event::Number(23, pushed),
            0x2CD /* BTN_TRIGGER_HAPPY14 */ => Event::Number(24, pushed),
            0x2CE /* BTN_TRIGGER_HAPPY15 */ => Event::Number(25, pushed),
            0x2CF /* BTN_TRIGGER_HAPPY16 */ => Event::Number(26, pushed),
            0x2D0 /* BTN_TRIGGER_HAPPY17 */ => Event::Number(27, pushed),
            0x2D1 /* BTN_TRIGGER_HAPPY18 */ => Event::Number(28, pushed),
            0x2D2 /* BTN_TRIGGER_HAPPY19 */ => Event::Number(29, pushed),
            0x2D3 /* BTN_TRIGGER_HAPPY20 */ => Event::Number(30, pushed),
            0x2D4 /* BTN_TRIGGER_HAPPY21 */ => Event::Number(31, pushed),
            0x2D5 /* BTN_TRIGGER_HAPPY22 */ => Event::Number(32, pushed),
            0x2D6 /* BTN_TRIGGER_HAPPY23 */ => Event::Number(33, pushed),
            0x2D7 /* BTN_TRIGGER_HAPPY24 */ => Event::Number(34, pushed),
            0x2D8 /* BTN_TRIGGER_HAPPY25 */ => Event::Number(35, pushed),
            0x2D9 /* BTN_TRIGGER_HAPPY26 */ => Event::Number(36, pushed),
            0x2DA /* BTN_TRIGGER_HAPPY27 */ => Event::Number(37, pushed),
            0x2DB /* BTN_TRIGGER_HAPPY28 */ => Event::Number(38, pushed),
            0x2DC /* BTN_TRIGGER_HAPPY29 */ => Event::Number(39, pushed),
            0x2DD /* BTN_TRIGGER_HAPPY30 */ => Event::Number(40, pushed),
            0x2DE /* BTN_TRIGGER_HAPPY31 */ => Event::Number(41, pushed),
            0x2DF /* BTN_TRIGGER_HAPPY32 */ => Event::Number(42, pushed),
            0x2E0 /* BTN_TRIGGER_HAPPY33 */ => Event::Number(43, pushed),
            0x2E1 /* BTN_TRIGGER_HAPPY34 */ => Event::Number(44, pushed),
            0x2E2 /* BTN_TRIGGER_HAPPY35 */ => Event::Number(45, pushed),
            0x2E3 /* BTN_TRIGGER_HAPPY36 */ => Event::Number(46, pushed),
            0x2E4 /* BTN_TRIGGER_HAPPY37 */ => Event::Number(47, pushed),
            0x2E5 /* BTN_TRIGGER_HAPPY38 */ => Event::Number(48, pushed),
            0x2E6 /* BTN_TRIGGER_HAPPY39 */ => Event::Number(49, pushed),
            0x2E7 /* BTN_TRIGGER_HAPPY40 */ => Event::Number(50, pushed),
            other => unimplemented!("unknown button: {other}"),
        })
    }

    fn from_rel_event(input_event: &input_event) -> Option<Self> {
        let axis = input_event.code;
        let value = input_event.value.try_into().ok()?;

        Some(match axis as _ {
            REL_X => Event::MouseX(value),
            REL_Y => Event::MouseY(value),
            REL_Z => todo!(),
            REL_RX => todo!(),
            REL_RY => todo!(),
            REL_RZ => todo!(),
            REL_HWHEEL => todo!(),
            REL_DIAL => todo!(),
            REL_WHEEL => todo!(),
            REL_MISC => todo!(),
            unknown => unimplemented!("unknown rel: {unknown}"),
        })
    }

    fn from_abs_event(input_event: &input_event, state: &mut u8) -> Option<Self> {
        let axis = input_event.code;
        let raw_val = input_event.value;
        let value = raw_val.try_into().ok()?;

        const POV_HOR_OFS: u8 = 0;
        const POV_VER_OFS: u8 = 1;
        const HAT_HOR_OFS: u8 = 2;
        const HAT_VER_OFS: u8 = 3;
        const TRIM_HOR_OFS: u8 = 4;
        const TRIM_VER_OFS: u8 = 5;
        const MIC_HOR_OFS: u8 = 6;
        const MIC_VER_OFS: u8 = 7;

        Some(match axis as _ {
            0x00..=0x0F => match axis as _ {
                ABS_X => Event::JoyX(value),
                ABS_Y => Event::JoyY(value),
                ABS_Z => Event::JoyZ(value),
                ABS_RX => Event::CamX(value),
                ABS_RY => Event::CamY(value),
                ABS_RZ => Event::CamZ(value),
                ABS_THROTTLE => Event::Throttle(value),
                ABS_RUDDER => Event::Rudder(value),
                ABS_WHEEL => Event::Wheel(value),
                ABS_GAS => Event::Gas(value),
                ABS_BRAKE => Event::Brake(value),
                ABS_UNKNOWN0 => Event::Slew(value),
                ABS_UNKNOWN1 => Event::ThrottleL(value),
                ABS_UNKNOWN2 => Event::ThrottleR(value),
                ABS_UNKNOWN3 => Event::ScrollX(value),
                ABS_UNKNOWN4 => Event::ScrollY(value),
                _ => unreachable!()
            }
            0x10..=0x17 => {
                use std::cmp::Ordering;
                match (axis as _, raw_val.cmp(&0)) {
                    (ABS_HAT0X, Ordering::Greater) => {
                        apply_bitmask_greater::<POV_HOR_OFS>(state);
                        Event::PovRight(true)
                    }
                    (ABS_HAT0X, Ordering::Less) => Event::PovLeft(true),
                    (ABS_HAT0X, Ordering::Equal) => {
                        let out = apply_bitmask_eq::<POV_HOR_OFS>(state);
                        if out {
                            Event::PovRight(false)
                        }else {
                            Event::PovLeft(false)
                        }
                    }

                    (ABS_HAT0Y, Ordering::Greater) => {
                        apply_bitmask_greater::<POV_VER_OFS>(state);
                        Event::PovDown(true)
                    }
                    (ABS_HAT0Y, Ordering::Less) => Event::PovUp(true),
                    (ABS_HAT0Y, Ordering::Equal) => {
                        let out = apply_bitmask_eq::<POV_VER_OFS>(state);
                        if out {
                            Event::PovDown(false)
                        }else {
                            Event::PovUp(false)
                        }
                    }

                    (ABS_HAT1X, Ordering::Greater) => {
                        apply_bitmask_greater::<HAT_HOR_OFS>(state);
                        Event::HatRight(true)
                    }
                    (ABS_HAT1X, Ordering::Less) => Event::HatLeft(true),
                    (ABS_HAT1X, Ordering::Equal) => {
                        let out = apply_bitmask_eq::<HAT_HOR_OFS>(state);
                        if out {
                            Event::HatRight(false)
                        }else {
                            Event::HatLeft(false)
                        }
                    }

                    (ABS_HAT1Y, Ordering::Greater) => {
                        apply_bitmask_greater::<HAT_VER_OFS>(state);
                        Event::HatDown(true)
                    }
                    (ABS_HAT1Y, Ordering::Less) => Event::HatUp(true),
                    (ABS_HAT1Y, Ordering::Equal) => {
                        let out = apply_bitmask_eq::<HAT_VER_OFS>(state);
                        if out {
                            Event::HatDown(false)
                        }else {
                            Event::HatUp(false)
                        }
                    }

                    (ABS_HAT2X, Ordering::Greater) => {
                        apply_bitmask_greater::<TRIM_HOR_OFS>(state);
                        Event::TrimRight(true)
                    }
                    (ABS_HAT2X, Ordering::Less) => Event::TrimLeft(true),
                    (ABS_HAT2X, Ordering::Equal) => {
                        let out = apply_bitmask_eq::<TRIM_HOR_OFS>(state);
                        if out {
                            Event::TrimRight(false)
                        }else {
                            Event::TrimLeft(false)
                        }
                    }

                    (ABS_HAT2Y, Ordering::Greater) => {
                        apply_bitmask_greater::<TRIM_VER_OFS>(state);
                        Event::TrimDown(true)
                    }
                    (ABS_HAT2Y, Ordering::Less) => Event::TrimUp(true),
                    (ABS_HAT2Y, Ordering::Equal) => {
                        let out = apply_bitmask_eq::<TRIM_VER_OFS>(state);
                        if out {
                            Event::TrimDown(false)
                        }else {
                            Event::TrimUp(false)
                        }
                    }

                    (ABS_HAT3X, Ordering::Greater) => {
                        apply_bitmask_greater::<MIC_HOR_OFS>(state);
                        Event::MicRight(true)
                    }
                    (ABS_HAT3X, Ordering::Less) => Event::MicLeft(true),
                    (ABS_HAT3X, Ordering::Equal) => {
                        let out = apply_bitmask_eq::<MIC_HOR_OFS>(state);
                        if out {
                            Event::MicRight(false)
                        }else {
                            Event::MicLeft(false)
                        }
                    }

                    (ABS_HAT3Y, Ordering::Greater) => {
                        apply_bitmask_greater::<MIC_VER_OFS>(state);
                        Event::MicDown(true)
                    }
                    (ABS_HAT3Y, Ordering::Less) => Event::MicUp(true),
                    (ABS_HAT3Y, Ordering::Equal) => {
                        let out = apply_bitmask_eq::<MIC_VER_OFS>(state);
                        if out {
                            Event::MicDown(false)
                        }else {
                            Event::MicUp(false)
                        }
                    }
                    _ => unreachable!(),
                }
            }
            unknown => unimplemented!("unknown code: {unknown}"),
        })
    }
}

fn apply_bitmask_greater<const BIT_OFFSET: u8>(state: &mut u8) {
    let mask = 1 << BIT_OFFSET;
    *state |= mask;
}

fn apply_bitmask_eq<const BIT_OFFSET: u8>(state: &mut u8) -> bool {
    let mask = 1 << BIT_OFFSET;
    let out = (*state & mask) > 0;
    *state &= !mask;
    out
}
