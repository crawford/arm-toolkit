# [ doc = "GPIO" ]
# [ repr ( C ) ]
# [ allow ( dead_code ) ]
pub struct Gpio {
    # [ doc = "0x00 - Port Control Register" ]
    pub pa_ctrl: PaCtrl,
    # [ doc = "0x04 - Port Pin Mode Low Register" ]
    pub pa_model: PaModel,
    # [ doc = "0x08 - Port Pin Mode High Register" ]
    pub pa_modeh: PaModeh,
    # [ doc = "0x0c - Port Data Out Register" ]
    pub pa_dout: PaDout,
    # [ doc = "0x10 - Port Data Out Set Register" ]
    pub pa_doutset: PaDoutset,
    # [ doc = "0x14 - Port Data Out Clear Register" ]
    pub pa_doutclr: PaDoutclr,
    # [ doc = "0x18 - Port Data Out Toggle Register" ]
    pub pa_douttgl: PaDouttgl,
    # [ doc = "0x1c - Port Data In Register" ]
    pub pa_din: PaDin,
    # [ doc = "0x20 - Port Unlocked Pins Register" ]
    pub pa_pinlockn: PaPinlockn,
    # [ doc = "0x24 - Port Control Register" ]
    pub pb_ctrl: PbCtrl,
    # [ doc = "0x28 - Port Pin Mode Low Register" ]
    pub pb_model: PbModel,
    # [ doc = "0x2c - Port Pin Mode High Register" ]
    pub pb_modeh: PbModeh,
    # [ doc = "0x30 - Port Data Out Register" ]
    pub pb_dout: PbDout,
    # [ doc = "0x34 - Port Data Out Set Register" ]
    pub pb_doutset: PbDoutset,
    # [ doc = "0x38 - Port Data Out Clear Register" ]
    pub pb_doutclr: PbDoutclr,
    # [ doc = "0x3c - Port Data Out Toggle Register" ]
    pub pb_douttgl: PbDouttgl,
    # [ doc = "0x40 - Port Data In Register" ]
    pub pb_din: PbDin,
    # [ doc = "0x44 - Port Unlocked Pins Register" ]
    pub pb_pinlockn: PbPinlockn,
    # [ doc = "0x48 - Port Control Register" ]
    pub pc_ctrl: PcCtrl,
    # [ doc = "0x4c - Port Pin Mode Low Register" ]
    pub pc_model: PcModel,
    # [ doc = "0x50 - Port Pin Mode High Register" ]
    pub pc_modeh: PcModeh,
    # [ doc = "0x54 - Port Data Out Register" ]
    pub pc_dout: PcDout,
    # [ doc = "0x58 - Port Data Out Set Register" ]
    pub pc_doutset: PcDoutset,
    # [ doc = "0x5c - Port Data Out Clear Register" ]
    pub pc_doutclr: PcDoutclr,
    # [ doc = "0x60 - Port Data Out Toggle Register" ]
    pub pc_douttgl: PcDouttgl,
    # [ doc = "0x64 - Port Data In Register" ]
    pub pc_din: PcDin,
    # [ doc = "0x68 - Port Unlocked Pins Register" ]
    pub pc_pinlockn: PcPinlockn,
    # [ doc = "0x6c - Port Control Register" ]
    pub pd_ctrl: PdCtrl,
    # [ doc = "0x70 - Port Pin Mode Low Register" ]
    pub pd_model: PdModel,
    # [ doc = "0x74 - Port Pin Mode High Register" ]
    pub pd_modeh: PdModeh,
    # [ doc = "0x78 - Port Data Out Register" ]
    pub pd_dout: PdDout,
    # [ doc = "0x7c - Port Data Out Set Register" ]
    pub pd_doutset: PdDoutset,
    # [ doc = "0x80 - Port Data Out Clear Register" ]
    pub pd_doutclr: PdDoutclr,
    # [ doc = "0x84 - Port Data Out Toggle Register" ]
    pub pd_douttgl: PdDouttgl,
    # [ doc = "0x88 - Port Data In Register" ]
    pub pd_din: PdDin,
    # [ doc = "0x8c - Port Unlocked Pins Register" ]
    pub pd_pinlockn: PdPinlockn,
    # [ doc = "0x90 - Port Control Register" ]
    pub pe_ctrl: PeCtrl,
    # [ doc = "0x94 - Port Pin Mode Low Register" ]
    pub pe_model: PeModel,
    # [ doc = "0x98 - Port Pin Mode High Register" ]
    pub pe_modeh: PeModeh,
    # [ doc = "0x9c - Port Data Out Register" ]
    pub pe_dout: PeDout,
    # [ doc = "0xa0 - Port Data Out Set Register" ]
    pub pe_doutset: PeDoutset,
    # [ doc = "0xa4 - Port Data Out Clear Register" ]
    pub pe_doutclr: PeDoutclr,
    # [ doc = "0xa8 - Port Data Out Toggle Register" ]
    pub pe_douttgl: PeDouttgl,
    # [ doc = "0xac - Port Data In Register" ]
    pub pe_din: PeDin,
    # [ doc = "0xb0 - Port Unlocked Pins Register" ]
    pub pe_pinlockn: PePinlockn,
    # [ doc = "0xb4 - Port Control Register" ]
    pub pf_ctrl: PfCtrl,
    # [ doc = "0xb8 - Port Pin Mode Low Register" ]
    pub pf_model: PfModel,
    # [ doc = "0xbc - Port Pin Mode High Register" ]
    pub pf_modeh: PfModeh,
    # [ doc = "0xc0 - Port Data Out Register" ]
    pub pf_dout: PfDout,
    # [ doc = "0xc4 - Port Data Out Set Register" ]
    pub pf_doutset: PfDoutset,
    # [ doc = "0xc8 - Port Data Out Clear Register" ]
    pub pf_doutclr: PfDoutclr,
    # [ doc = "0xcc - Port Data Out Toggle Register" ]
    pub pf_douttgl: PfDouttgl,
    # [ doc = "0xd0 - Port Data In Register" ]
    pub pf_din: PfDin,
    # [ doc = "0xd4 - Port Unlocked Pins Register" ]
    pub pf_pinlockn: PfPinlockn,
    # [ doc ( hidden ) ]
    _reserved0: [u8; 40usize],
    # [ doc = "0x100 - External Interrupt Port Select Low Register" ]
    pub extipsell: Extipsell,
    # [ doc = "0x104 - External Interrupt Port Select High Register" ]
    pub extipselh: Extipselh,
    # [ doc = "0x108 - External Interrupt Rising Edge Trigger Register" ]
    pub extirise: Extirise,
    # [ doc = "0x10c - External Interrupt Falling Edge Trigger Register" ]
    pub extifall: Extifall,
    # [ doc = "0x110 - Interrupt Enable Register" ]
    pub ien: Ien,
    # [ doc = "0x114 - Interrupt Flag Register" ]
    pub if_: If,
    # [ doc = "0x118 - Interrupt Flag Set Register" ]
    pub ifs: Ifs,
    # [ doc = "0x11c - Interrupt Flag Clear Register" ]
    pub ifc: Ifc,
    # [ doc = "0x120 - I/O Routing Register" ]
    pub route: Route,
    # [ doc = "0x124 - Input Sense Register" ]
    pub insense: Insense,
    # [ doc = "0x128 - Configuration Lock Register" ]
    pub lock: Lock,
    # [ doc = "0x12c - GPIO Control Register" ]
    pub ctrl: Ctrl,
    # [ doc = "0x130 - GPIO Command Register" ]
    pub cmd: Cmd,
    # [ doc = "0x134 - EM4 Wake-up Enable Register" ]
    pub em4wuen: Em4wuen,
    # [ doc = "0x138 - EM4 Wake-up Polarity Register" ]
    pub em4wupol: Em4wupol,
    # [ doc = "0x13c - EM4 Wake-up Cause Register" ]
    pub em4wucause: Em4wucause,
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaCtrl {
    register: ::volatile_register::RW<u32>,
}

impl PaCtrl {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PaCtrlR, &'w mut PaCtrlW) -> &'w mut PaCtrlW
    {
        let bits = self.register.read();
        let r = PaCtrlR { bits: bits };
        let mut w = PaCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PaCtrlR {
        PaCtrlR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PaCtrlW) -> &mut PaCtrlW
    {
        let mut w = PaCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaCtrlR {
    bits: u32,
}

impl PaCtrlR {
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 3;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaCtrlW {
    bits: u32,
}

impl PaCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PaCtrlW { bits: 0 }
    }
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaModel {
    register: ::volatile_register::RW<u32>,
}

impl PaModel {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PaModelR, &'w mut PaModelW) -> &'w mut PaModelW
    {
        let bits = self.register.read();
        let r = PaModelR { bits: bits };
        let mut w = PaModelW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PaModelR {
        PaModelR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PaModelW) -> &mut PaModelW
    {
        let mut w = PaModelW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaModelR {
    bits: u32,
}

impl PaModelR {
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaModelW {
    bits: u32,
}

impl PaModelW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PaModelW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaModeh {
    register: ::volatile_register::RW<u32>,
}

impl PaModeh {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PaModehR, &'w mut PaModehW) -> &'w mut PaModehW
    {
        let bits = self.register.read();
        let r = PaModehR { bits: bits };
        let mut w = PaModehW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PaModehR {
        PaModehR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PaModehW) -> &mut PaModehW
    {
        let mut w = PaModehW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaModehR {
    bits: u32,
}

impl PaModehR {
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaModehW {
    bits: u32,
}

impl PaModehW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PaModehW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaDout {
    register: ::volatile_register::RW<u32>,
}

impl PaDout {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PaDoutR, &'w mut PaDoutW) -> &'w mut PaDoutW
    {
        let bits = self.register.read();
        let r = PaDoutR { bits: bits };
        let mut w = PaDoutW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PaDoutR {
        PaDoutR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PaDoutW) -> &mut PaDoutW
    {
        let mut w = PaDoutW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaDoutR {
    bits: u32,
}

impl PaDoutR {
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaDoutW {
    bits: u32,
}

impl PaDoutW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PaDoutW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaDoutset {
    register: ::volatile_register::WO<u32>,
}

impl PaDoutset {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PaDoutsetW) -> &mut PaDoutsetW
    {
        let mut w = PaDoutsetW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaDoutsetW {
    bits: u32,
}

impl PaDoutsetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PaDoutsetW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Set" ]
    # [ allow ( dead_code ) ]
    pub fn doutset(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaDoutclr {
    register: ::volatile_register::WO<u32>,
}

impl PaDoutclr {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PaDoutclrW) -> &mut PaDoutclrW
    {
        let mut w = PaDoutclrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaDoutclrW {
    bits: u32,
}

impl PaDoutclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PaDoutclrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Clear" ]
    # [ allow ( dead_code ) ]
    pub fn doutclr(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaDouttgl {
    register: ::volatile_register::WO<u32>,
}

impl PaDouttgl {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PaDouttglW) -> &mut PaDouttglW
    {
        let mut w = PaDouttglW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaDouttglW {
    bits: u32,
}

impl PaDouttglW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PaDouttglW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Toggle" ]
    # [ allow ( dead_code ) ]
    pub fn douttgl(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaDin {
    register: ::volatile_register::RO<u32>,
}

impl PaDin {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PaDinR {
        PaDinR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaDinR {
    bits: u32,
}

impl PaDinR {
    # [ doc = "Bits 0:15 - Data In" ]
    # [ allow ( dead_code ) ]
    pub fn din(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaPinlockn {
    register: ::volatile_register::RW<u32>,
}

impl PaPinlockn {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PaPinlocknR, &'w mut PaPinlocknW) -> &'w mut PaPinlocknW
    {
        let bits = self.register.read();
        let r = PaPinlocknR { bits: bits };
        let mut w = PaPinlocknW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PaPinlocknR {
        PaPinlocknR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PaPinlocknW) -> &mut PaPinlocknW
    {
        let mut w = PaPinlocknW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaPinlocknR {
    bits: u32,
}

impl PaPinlocknR {
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PaPinlocknW {
    bits: u32,
}

impl PaPinlocknW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PaPinlocknW { bits: 65535 }
    }
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbCtrl {
    register: ::volatile_register::RW<u32>,
}

impl PbCtrl {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PbCtrlR, &'w mut PbCtrlW) -> &'w mut PbCtrlW
    {
        let bits = self.register.read();
        let r = PbCtrlR { bits: bits };
        let mut w = PbCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PbCtrlR {
        PbCtrlR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PbCtrlW) -> &mut PbCtrlW
    {
        let mut w = PbCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbCtrlR {
    bits: u32,
}

impl PbCtrlR {
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 3;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbCtrlW {
    bits: u32,
}

impl PbCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PbCtrlW { bits: 0 }
    }
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbModel {
    register: ::volatile_register::RW<u32>,
}

impl PbModel {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PbModelR, &'w mut PbModelW) -> &'w mut PbModelW
    {
        let bits = self.register.read();
        let r = PbModelR { bits: bits };
        let mut w = PbModelW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PbModelR {
        PbModelR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PbModelW) -> &mut PbModelW
    {
        let mut w = PbModelW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbModelR {
    bits: u32,
}

impl PbModelR {
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbModelW {
    bits: u32,
}

impl PbModelW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PbModelW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbModeh {
    register: ::volatile_register::RW<u32>,
}

impl PbModeh {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PbModehR, &'w mut PbModehW) -> &'w mut PbModehW
    {
        let bits = self.register.read();
        let r = PbModehR { bits: bits };
        let mut w = PbModehW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PbModehR {
        PbModehR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PbModehW) -> &mut PbModehW
    {
        let mut w = PbModehW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbModehR {
    bits: u32,
}

impl PbModehR {
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbModehW {
    bits: u32,
}

impl PbModehW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PbModehW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbDout {
    register: ::volatile_register::RW<u32>,
}

impl PbDout {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PbDoutR, &'w mut PbDoutW) -> &'w mut PbDoutW
    {
        let bits = self.register.read();
        let r = PbDoutR { bits: bits };
        let mut w = PbDoutW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PbDoutR {
        PbDoutR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PbDoutW) -> &mut PbDoutW
    {
        let mut w = PbDoutW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbDoutR {
    bits: u32,
}

impl PbDoutR {
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbDoutW {
    bits: u32,
}

impl PbDoutW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PbDoutW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbDoutset {
    register: ::volatile_register::WO<u32>,
}

impl PbDoutset {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PbDoutsetW) -> &mut PbDoutsetW
    {
        let mut w = PbDoutsetW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbDoutsetW {
    bits: u32,
}

impl PbDoutsetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PbDoutsetW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Set" ]
    # [ allow ( dead_code ) ]
    pub fn doutset(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbDoutclr {
    register: ::volatile_register::WO<u32>,
}

impl PbDoutclr {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PbDoutclrW) -> &mut PbDoutclrW
    {
        let mut w = PbDoutclrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbDoutclrW {
    bits: u32,
}

impl PbDoutclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PbDoutclrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Clear" ]
    # [ allow ( dead_code ) ]
    pub fn doutclr(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbDouttgl {
    register: ::volatile_register::WO<u32>,
}

impl PbDouttgl {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PbDouttglW) -> &mut PbDouttglW
    {
        let mut w = PbDouttglW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbDouttglW {
    bits: u32,
}

impl PbDouttglW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PbDouttglW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Toggle" ]
    # [ allow ( dead_code ) ]
    pub fn douttgl(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbDin {
    register: ::volatile_register::RO<u32>,
}

impl PbDin {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PbDinR {
        PbDinR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbDinR {
    bits: u32,
}

impl PbDinR {
    # [ doc = "Bits 0:15 - Data In" ]
    # [ allow ( dead_code ) ]
    pub fn din(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbPinlockn {
    register: ::volatile_register::RW<u32>,
}

impl PbPinlockn {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PbPinlocknR, &'w mut PbPinlocknW) -> &'w mut PbPinlocknW
    {
        let bits = self.register.read();
        let r = PbPinlocknR { bits: bits };
        let mut w = PbPinlocknW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PbPinlocknR {
        PbPinlocknR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PbPinlocknW) -> &mut PbPinlocknW
    {
        let mut w = PbPinlocknW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbPinlocknR {
    bits: u32,
}

impl PbPinlocknR {
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PbPinlocknW {
    bits: u32,
}

impl PbPinlocknW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PbPinlocknW { bits: 65535 }
    }
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcCtrl {
    register: ::volatile_register::RW<u32>,
}

impl PcCtrl {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PcCtrlR, &'w mut PcCtrlW) -> &'w mut PcCtrlW
    {
        let bits = self.register.read();
        let r = PcCtrlR { bits: bits };
        let mut w = PcCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PcCtrlR {
        PcCtrlR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PcCtrlW) -> &mut PcCtrlW
    {
        let mut w = PcCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcCtrlR {
    bits: u32,
}

impl PcCtrlR {
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 3;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcCtrlW {
    bits: u32,
}

impl PcCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcCtrlW { bits: 0 }
    }
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcModel {
    register: ::volatile_register::RW<u32>,
}

impl PcModel {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PcModelR, &'w mut PcModelW) -> &'w mut PcModelW
    {
        let bits = self.register.read();
        let r = PcModelR { bits: bits };
        let mut w = PcModelW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PcModelR {
        PcModelR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PcModelW) -> &mut PcModelW
    {
        let mut w = PcModelW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcModelR {
    bits: u32,
}

impl PcModelR {
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcModelW {
    bits: u32,
}

impl PcModelW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcModelW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcModeh {
    register: ::volatile_register::RW<u32>,
}

impl PcModeh {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PcModehR, &'w mut PcModehW) -> &'w mut PcModehW
    {
        let bits = self.register.read();
        let r = PcModehR { bits: bits };
        let mut w = PcModehW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PcModehR {
        PcModehR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PcModehW) -> &mut PcModehW
    {
        let mut w = PcModehW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcModehR {
    bits: u32,
}

impl PcModehR {
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcModehW {
    bits: u32,
}

impl PcModehW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcModehW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcDout {
    register: ::volatile_register::RW<u32>,
}

impl PcDout {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PcDoutR, &'w mut PcDoutW) -> &'w mut PcDoutW
    {
        let bits = self.register.read();
        let r = PcDoutR { bits: bits };
        let mut w = PcDoutW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PcDoutR {
        PcDoutR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PcDoutW) -> &mut PcDoutW
    {
        let mut w = PcDoutW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcDoutR {
    bits: u32,
}

impl PcDoutR {
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcDoutW {
    bits: u32,
}

impl PcDoutW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcDoutW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcDoutset {
    register: ::volatile_register::WO<u32>,
}

impl PcDoutset {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PcDoutsetW) -> &mut PcDoutsetW
    {
        let mut w = PcDoutsetW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcDoutsetW {
    bits: u32,
}

impl PcDoutsetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcDoutsetW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Set" ]
    # [ allow ( dead_code ) ]
    pub fn doutset(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcDoutclr {
    register: ::volatile_register::WO<u32>,
}

impl PcDoutclr {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PcDoutclrW) -> &mut PcDoutclrW
    {
        let mut w = PcDoutclrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcDoutclrW {
    bits: u32,
}

impl PcDoutclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcDoutclrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Clear" ]
    # [ allow ( dead_code ) ]
    pub fn doutclr(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcDouttgl {
    register: ::volatile_register::WO<u32>,
}

impl PcDouttgl {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PcDouttglW) -> &mut PcDouttglW
    {
        let mut w = PcDouttglW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcDouttglW {
    bits: u32,
}

impl PcDouttglW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcDouttglW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Toggle" ]
    # [ allow ( dead_code ) ]
    pub fn douttgl(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcDin {
    register: ::volatile_register::RO<u32>,
}

impl PcDin {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PcDinR {
        PcDinR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcDinR {
    bits: u32,
}

impl PcDinR {
    # [ doc = "Bits 0:15 - Data In" ]
    # [ allow ( dead_code ) ]
    pub fn din(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcPinlockn {
    register: ::volatile_register::RW<u32>,
}

impl PcPinlockn {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PcPinlocknR, &'w mut PcPinlocknW) -> &'w mut PcPinlocknW
    {
        let bits = self.register.read();
        let r = PcPinlocknR { bits: bits };
        let mut w = PcPinlocknW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PcPinlocknR {
        PcPinlocknR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PcPinlocknW) -> &mut PcPinlocknW
    {
        let mut w = PcPinlocknW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcPinlocknR {
    bits: u32,
}

impl PcPinlocknR {
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PcPinlocknW {
    bits: u32,
}

impl PcPinlocknW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcPinlocknW { bits: 65535 }
    }
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdCtrl {
    register: ::volatile_register::RW<u32>,
}

impl PdCtrl {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdCtrlR, &'w mut PdCtrlW) -> &'w mut PdCtrlW
    {
        let bits = self.register.read();
        let r = PdCtrlR { bits: bits };
        let mut w = PdCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PdCtrlR {
        PdCtrlR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdCtrlW) -> &mut PdCtrlW
    {
        let mut w = PdCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdCtrlR {
    bits: u32,
}

impl PdCtrlR {
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 3;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdCtrlW {
    bits: u32,
}

impl PdCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdCtrlW { bits: 0 }
    }
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdModel {
    register: ::volatile_register::RW<u32>,
}

impl PdModel {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdModelR, &'w mut PdModelW) -> &'w mut PdModelW
    {
        let bits = self.register.read();
        let r = PdModelR { bits: bits };
        let mut w = PdModelW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PdModelR {
        PdModelR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdModelW) -> &mut PdModelW
    {
        let mut w = PdModelW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdModelR {
    bits: u32,
}

impl PdModelR {
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdModelW {
    bits: u32,
}

impl PdModelW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdModelW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdModeh {
    register: ::volatile_register::RW<u32>,
}

impl PdModeh {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdModehR, &'w mut PdModehW) -> &'w mut PdModehW
    {
        let bits = self.register.read();
        let r = PdModehR { bits: bits };
        let mut w = PdModehW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PdModehR {
        PdModehR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdModehW) -> &mut PdModehW
    {
        let mut w = PdModehW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdModehR {
    bits: u32,
}

impl PdModehR {
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdModehW {
    bits: u32,
}

impl PdModehW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdModehW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdDout {
    register: ::volatile_register::RW<u32>,
}

impl PdDout {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdDoutR, &'w mut PdDoutW) -> &'w mut PdDoutW
    {
        let bits = self.register.read();
        let r = PdDoutR { bits: bits };
        let mut w = PdDoutW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PdDoutR {
        PdDoutR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdDoutW) -> &mut PdDoutW
    {
        let mut w = PdDoutW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdDoutR {
    bits: u32,
}

impl PdDoutR {
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdDoutW {
    bits: u32,
}

impl PdDoutW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdDoutW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdDoutset {
    register: ::volatile_register::WO<u32>,
}

impl PdDoutset {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PdDoutsetW) -> &mut PdDoutsetW
    {
        let mut w = PdDoutsetW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdDoutsetW {
    bits: u32,
}

impl PdDoutsetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdDoutsetW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Set" ]
    # [ allow ( dead_code ) ]
    pub fn doutset(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdDoutclr {
    register: ::volatile_register::WO<u32>,
}

impl PdDoutclr {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PdDoutclrW) -> &mut PdDoutclrW
    {
        let mut w = PdDoutclrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdDoutclrW {
    bits: u32,
}

impl PdDoutclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdDoutclrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Clear" ]
    # [ allow ( dead_code ) ]
    pub fn doutclr(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdDouttgl {
    register: ::volatile_register::WO<u32>,
}

impl PdDouttgl {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PdDouttglW) -> &mut PdDouttglW
    {
        let mut w = PdDouttglW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdDouttglW {
    bits: u32,
}

impl PdDouttglW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdDouttglW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Toggle" ]
    # [ allow ( dead_code ) ]
    pub fn douttgl(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdDin {
    register: ::volatile_register::RO<u32>,
}

impl PdDin {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PdDinR {
        PdDinR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdDinR {
    bits: u32,
}

impl PdDinR {
    # [ doc = "Bits 0:15 - Data In" ]
    # [ allow ( dead_code ) ]
    pub fn din(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdPinlockn {
    register: ::volatile_register::RW<u32>,
}

impl PdPinlockn {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdPinlocknR, &'w mut PdPinlocknW) -> &'w mut PdPinlocknW
    {
        let bits = self.register.read();
        let r = PdPinlocknR { bits: bits };
        let mut w = PdPinlocknW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PdPinlocknR {
        PdPinlocknR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdPinlocknW) -> &mut PdPinlocknW
    {
        let mut w = PdPinlocknW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdPinlocknR {
    bits: u32,
}

impl PdPinlocknR {
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PdPinlocknW {
    bits: u32,
}

impl PdPinlocknW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdPinlocknW { bits: 65535 }
    }
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeCtrl {
    register: ::volatile_register::RW<u32>,
}

impl PeCtrl {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PeCtrlR, &'w mut PeCtrlW) -> &'w mut PeCtrlW
    {
        let bits = self.register.read();
        let r = PeCtrlR { bits: bits };
        let mut w = PeCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PeCtrlR {
        PeCtrlR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PeCtrlW) -> &mut PeCtrlW
    {
        let mut w = PeCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeCtrlR {
    bits: u32,
}

impl PeCtrlR {
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 3;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeCtrlW {
    bits: u32,
}

impl PeCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PeCtrlW { bits: 0 }
    }
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeModel {
    register: ::volatile_register::RW<u32>,
}

impl PeModel {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PeModelR, &'w mut PeModelW) -> &'w mut PeModelW
    {
        let bits = self.register.read();
        let r = PeModelR { bits: bits };
        let mut w = PeModelW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PeModelR {
        PeModelR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PeModelW) -> &mut PeModelW
    {
        let mut w = PeModelW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeModelR {
    bits: u32,
}

impl PeModelR {
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeModelW {
    bits: u32,
}

impl PeModelW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PeModelW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeModeh {
    register: ::volatile_register::RW<u32>,
}

impl PeModeh {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PeModehR, &'w mut PeModehW) -> &'w mut PeModehW
    {
        let bits = self.register.read();
        let r = PeModehR { bits: bits };
        let mut w = PeModehW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PeModehR {
        PeModehR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PeModehW) -> &mut PeModehW
    {
        let mut w = PeModehW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeModehR {
    bits: u32,
}

impl PeModehR {
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeModehW {
    bits: u32,
}

impl PeModehW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PeModehW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeDout {
    register: ::volatile_register::RW<u32>,
}

impl PeDout {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PeDoutR, &'w mut PeDoutW) -> &'w mut PeDoutW
    {
        let bits = self.register.read();
        let r = PeDoutR { bits: bits };
        let mut w = PeDoutW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PeDoutR {
        PeDoutR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PeDoutW) -> &mut PeDoutW
    {
        let mut w = PeDoutW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeDoutR {
    bits: u32,
}

impl PeDoutR {
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeDoutW {
    bits: u32,
}

impl PeDoutW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PeDoutW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeDoutset {
    register: ::volatile_register::WO<u32>,
}

impl PeDoutset {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PeDoutsetW) -> &mut PeDoutsetW
    {
        let mut w = PeDoutsetW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeDoutsetW {
    bits: u32,
}

impl PeDoutsetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PeDoutsetW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Set" ]
    # [ allow ( dead_code ) ]
    pub fn doutset(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeDoutclr {
    register: ::volatile_register::WO<u32>,
}

impl PeDoutclr {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PeDoutclrW) -> &mut PeDoutclrW
    {
        let mut w = PeDoutclrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeDoutclrW {
    bits: u32,
}

impl PeDoutclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PeDoutclrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Clear" ]
    # [ allow ( dead_code ) ]
    pub fn doutclr(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeDouttgl {
    register: ::volatile_register::WO<u32>,
}

impl PeDouttgl {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PeDouttglW) -> &mut PeDouttglW
    {
        let mut w = PeDouttglW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeDouttglW {
    bits: u32,
}

impl PeDouttglW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PeDouttglW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Toggle" ]
    # [ allow ( dead_code ) ]
    pub fn douttgl(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeDin {
    register: ::volatile_register::RO<u32>,
}

impl PeDin {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PeDinR {
        PeDinR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PeDinR {
    bits: u32,
}

impl PeDinR {
    # [ doc = "Bits 0:15 - Data In" ]
    # [ allow ( dead_code ) ]
    pub fn din(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PePinlockn {
    register: ::volatile_register::RW<u32>,
}

impl PePinlockn {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PePinlocknR, &'w mut PePinlocknW) -> &'w mut PePinlocknW
    {
        let bits = self.register.read();
        let r = PePinlocknR { bits: bits };
        let mut w = PePinlocknW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PePinlocknR {
        PePinlocknR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PePinlocknW) -> &mut PePinlocknW
    {
        let mut w = PePinlocknW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PePinlocknR {
    bits: u32,
}

impl PePinlocknR {
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PePinlocknW {
    bits: u32,
}

impl PePinlocknW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PePinlocknW { bits: 65535 }
    }
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfCtrl {
    register: ::volatile_register::RW<u32>,
}

impl PfCtrl {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PfCtrlR, &'w mut PfCtrlW) -> &'w mut PfCtrlW
    {
        let bits = self.register.read();
        let r = PfCtrlR { bits: bits };
        let mut w = PfCtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PfCtrlR {
        PfCtrlR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PfCtrlW) -> &mut PfCtrlW
    {
        let mut w = PfCtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfCtrlR {
    bits: u32,
}

impl PfCtrlR {
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 3;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfCtrlW {
    bits: u32,
}

impl PfCtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PfCtrlW { bits: 0 }
    }
    # [ doc = "Bits 0:1 - Drive Mode Select" ]
    # [ allow ( dead_code ) ]
    pub fn drivemode(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfModel {
    register: ::volatile_register::RW<u32>,
}

impl PfModel {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PfModelR, &'w mut PfModelW) -> &'w mut PfModelW
    {
        let bits = self.register.read();
        let r = PfModelR { bits: bits };
        let mut w = PfModelW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PfModelR {
        PfModelR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PfModelW) -> &mut PfModelW
    {
        let mut w = PfModelW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfModelR {
    bits: u32,
}

impl PfModelR {
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfModelW {
    bits: u32,
}

impl PfModelW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PfModelW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 0 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode0(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 1 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode1(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 2 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode2(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 3 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode3(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 4 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode4(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 5 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode5(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 6 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode6(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 7 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode7(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfModeh {
    register: ::volatile_register::RW<u32>,
}

impl PfModeh {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PfModehR, &'w mut PfModehW) -> &'w mut PfModehW
    {
        let bits = self.register.read();
        let r = PfModehR { bits: bits };
        let mut w = PfModehW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PfModehR {
        PfModehR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PfModehW) -> &mut PfModehW
    {
        let mut w = PfModehW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfModehR {
    bits: u32,
}

impl PfModehR {
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 15;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfModehW {
    bits: u32,
}

impl PfModehW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PfModehW { bits: 0 }
    }
    # [ doc = "Bits 0:3 - Pin 8 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode8(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Pin 9 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode9(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Pin 10 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode10(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Pin 11 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode11(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - Pin 12 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode12(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - Pin 13 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode13(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - Pin 14 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode14(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:31 - Pin 15 Mode" ]
    # [ allow ( dead_code ) ]
    pub fn mode15(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfDout {
    register: ::volatile_register::RW<u32>,
}

impl PfDout {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PfDoutR, &'w mut PfDoutW) -> &'w mut PfDoutW
    {
        let bits = self.register.read();
        let r = PfDoutR { bits: bits };
        let mut w = PfDoutW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PfDoutR {
        PfDoutR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PfDoutW) -> &mut PfDoutW
    {
        let mut w = PfDoutW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfDoutR {
    bits: u32,
}

impl PfDoutR {
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfDoutW {
    bits: u32,
}

impl PfDoutW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PfDoutW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out" ]
    # [ allow ( dead_code ) ]
    pub fn dout(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfDoutset {
    register: ::volatile_register::WO<u32>,
}

impl PfDoutset {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PfDoutsetW) -> &mut PfDoutsetW
    {
        let mut w = PfDoutsetW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfDoutsetW {
    bits: u32,
}

impl PfDoutsetW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PfDoutsetW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Set" ]
    # [ allow ( dead_code ) ]
    pub fn doutset(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfDoutclr {
    register: ::volatile_register::WO<u32>,
}

impl PfDoutclr {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PfDoutclrW) -> &mut PfDoutclrW
    {
        let mut w = PfDoutclrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfDoutclrW {
    bits: u32,
}

impl PfDoutclrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PfDoutclrW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Clear" ]
    # [ allow ( dead_code ) ]
    pub fn doutclr(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfDouttgl {
    register: ::volatile_register::WO<u32>,
}

impl PfDouttgl {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PfDouttglW) -> &mut PfDouttglW
    {
        let mut w = PfDouttglW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfDouttglW {
    bits: u32,
}

impl PfDouttglW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PfDouttglW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Data Out Toggle" ]
    # [ allow ( dead_code ) ]
    pub fn douttgl(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfDin {
    register: ::volatile_register::RO<u32>,
}

impl PfDin {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PfDinR {
        PfDinR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfDinR {
    bits: u32,
}

impl PfDinR {
    # [ doc = "Bits 0:15 - Data In" ]
    # [ allow ( dead_code ) ]
    pub fn din(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfPinlockn {
    register: ::volatile_register::RW<u32>,
}

impl PfPinlockn {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PfPinlocknR, &'w mut PfPinlocknW) -> &'w mut PfPinlocknW
    {
        let bits = self.register.read();
        let r = PfPinlocknR { bits: bits };
        let mut w = PfPinlocknW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> PfPinlocknR {
        PfPinlocknR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PfPinlocknW) -> &mut PfPinlocknW
    {
        let mut w = PfPinlocknW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfPinlocknR {
    bits: u32,
}

impl PfPinlocknR {
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct PfPinlocknW {
    bits: u32,
}

impl PfPinlocknW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PfPinlocknW { bits: 65535 }
    }
    # [ doc = "Bits 0:15 - Unlocked Pins" ]
    # [ allow ( dead_code ) ]
    pub fn pinlockn(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Extipsell {
    register: ::volatile_register::RW<u32>,
}

impl Extipsell {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ExtipsellR, &'w mut ExtipsellW) -> &'w mut ExtipsellW
    {
        let bits = self.register.read();
        let r = ExtipsellR { bits: bits };
        let mut w = ExtipsellW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> ExtipsellR {
        ExtipsellR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ExtipsellW) -> &mut ExtipsellW
    {
        let mut w = ExtipsellW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct ExtipsellR {
    bits: u32,
}

impl ExtipsellR {
    # [ doc = "Bits 0:2 - External Interrupt 0 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel0(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:6 - External Interrupt 1 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel1(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:10 - External Interrupt 2 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel2(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:14 - External Interrupt 3 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel3(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:18 - External Interrupt 4 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel4(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:22 - External Interrupt 5 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel5(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:26 - External Interrupt 6 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel6(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:30 - External Interrupt 7 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel7(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct ExtipsellW {
    bits: u32,
}

impl ExtipsellW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ExtipsellW { bits: 0 }
    }
    # [ doc = "Bits 0:2 - External Interrupt 0 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel0(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:6 - External Interrupt 1 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel1(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:10 - External Interrupt 2 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel2(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:14 - External Interrupt 3 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel3(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:18 - External Interrupt 4 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel4(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:22 - External Interrupt 5 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel5(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:26 - External Interrupt 6 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel6(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:30 - External Interrupt 7 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel7(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Extipselh {
    register: ::volatile_register::RW<u32>,
}

impl Extipselh {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ExtipselhR, &'w mut ExtipselhW) -> &'w mut ExtipselhW
    {
        let bits = self.register.read();
        let r = ExtipselhR { bits: bits };
        let mut w = ExtipselhW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> ExtipselhR {
        ExtipselhR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ExtipselhW) -> &mut ExtipselhW
    {
        let mut w = ExtipselhW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct ExtipselhR {
    bits: u32,
}

impl ExtipselhR {
    # [ doc = "Bits 0:2 - External Interrupt 8 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel8(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:6 - External Interrupt 9 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel9(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:10 - External Interrupt 10 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel10(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:14 - External Interrupt 11 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel11(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:18 - External Interrupt 12 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel12(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:22 - External Interrupt 13 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel13(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:26 - External Interrupt 14 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel14(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 28:30 - External Interrupt 15 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel15(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 7;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct ExtipselhW {
    bits: u32,
}

impl ExtipselhW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ExtipselhW { bits: 0 }
    }
    # [ doc = "Bits 0:2 - External Interrupt 8 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel8(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:6 - External Interrupt 9 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel9(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 4u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:10 - External Interrupt 10 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel10(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:14 - External Interrupt 11 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel11(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:18 - External Interrupt 12 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel12(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:22 - External Interrupt 13 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel13(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 20u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:26 - External Interrupt 14 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel14(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 28:30 - External Interrupt 15 Port Select" ]
    # [ allow ( dead_code ) ]
    pub fn extipsel15(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 28u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Extirise {
    register: ::volatile_register::RW<u32>,
}

impl Extirise {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ExtiriseR, &'w mut ExtiriseW) -> &'w mut ExtiriseW
    {
        let bits = self.register.read();
        let r = ExtiriseR { bits: bits };
        let mut w = ExtiriseW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> ExtiriseR {
        ExtiriseR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ExtiriseW) -> &mut ExtiriseW
    {
        let mut w = ExtiriseW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct ExtiriseR {
    bits: u32,
}

impl ExtiriseR {
    # [ doc = "Bits 0:15 - External Interrupt n Rising Edge Trigger Enable" ]
    # [ allow ( dead_code ) ]
    pub fn extirise(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct ExtiriseW {
    bits: u32,
}

impl ExtiriseW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ExtiriseW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - External Interrupt n Rising Edge Trigger Enable" ]
    # [ allow ( dead_code ) ]
    pub fn extirise(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Extifall {
    register: ::volatile_register::RW<u32>,
}

impl Extifall {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ExtifallR, &'w mut ExtifallW) -> &'w mut ExtifallW
    {
        let bits = self.register.read();
        let r = ExtifallR { bits: bits };
        let mut w = ExtifallW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> ExtifallR {
        ExtifallR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ExtifallW) -> &mut ExtifallW
    {
        let mut w = ExtifallW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct ExtifallR {
    bits: u32,
}

impl ExtifallR {
    # [ doc = "Bits 0:15 - External Interrupt n Falling Edge Trigger Enable" ]
    # [ allow ( dead_code ) ]
    pub fn extifall(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct ExtifallW {
    bits: u32,
}

impl ExtifallW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ExtifallW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - External Interrupt n Falling Edge Trigger Enable" ]
    # [ allow ( dead_code ) ]
    pub fn extifall(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Ien {
    register: ::volatile_register::RW<u32>,
}

impl Ien {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IenR, &'w mut IenW) -> &'w mut IenW
    {
        let bits = self.register.read();
        let r = IenR { bits: bits };
        let mut w = IenW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> IenR {
        IenR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IenW) -> &mut IenW
    {
        let mut w = IenW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct IenR {
    bits: u32,
}

impl IenR {
    # [ doc = "Bits 0:15 - External Interrupt n Enable" ]
    # [ allow ( dead_code ) ]
    pub fn ext(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct IenW {
    bits: u32,
}

impl IenW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IenW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - External Interrupt n Enable" ]
    # [ allow ( dead_code ) ]
    pub fn ext(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct If {
    register: ::volatile_register::RO<u32>,
}

impl If {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> IfR {
        IfR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct IfR {
    bits: u32,
}

impl IfR {
    # [ doc = "Bits 0:15 - External Interrupt Flag n" ]
    # [ allow ( dead_code ) ]
    pub fn ext(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Ifs {
    register: ::volatile_register::WO<u32>,
}

impl Ifs {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut IfsW) -> &mut IfsW
    {
        let mut w = IfsW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct IfsW {
    bits: u32,
}

impl IfsW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IfsW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - External Interrupt Flag n Set" ]
    # [ allow ( dead_code ) ]
    pub fn ext(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Ifc {
    register: ::volatile_register::WO<u32>,
}

impl Ifc {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut IfcW) -> &mut IfcW
    {
        let mut w = IfcW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct IfcW {
    bits: u32,
}

impl IfcW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IfcW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - External Interrupt Flag Clear" ]
    # [ allow ( dead_code ) ]
    pub fn ext(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Route {
    register: ::volatile_register::RW<u32>,
}

impl Route {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&RouteR, &'w mut RouteW) -> &'w mut RouteW
    {
        let bits = self.register.read();
        let r = RouteR { bits: bits };
        let mut w = RouteW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> RouteR {
        RouteR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut RouteW) -> &mut RouteW
    {
        let mut w = RouteW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct RouteR {
    bits: u32,
}

impl RouteR {
    # [ doc = "Bit 0 - Serial Wire Clock Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn swclkpen(&self) -> bool {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Serial Wire Data Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn swdiopen(&self) -> bool {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Serial Wire Viewer Output Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn swopen(&self) -> bool {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:9 - I/O Location" ]
    # [ allow ( dead_code ) ]
    pub fn swlocation(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 3;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 12 - ETM Trace Clock Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn tclkpen(&self) -> bool {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - ETM Trace Data Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn td0pen(&self) -> bool {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - ETM Trace Data Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn td1pen(&self) -> bool {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - ETM Trace Data Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn td2pen(&self) -> bool {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - ETM Trace Data Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn td3pen(&self) -> bool {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 24:25 - I/O Location" ]
    # [ allow ( dead_code ) ]
    pub fn etmlocation(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 3;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct RouteW {
    bits: u32,
}

impl RouteW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        RouteW { bits: 3 }
    }
    # [ doc = "Bit 0 - Serial Wire Clock Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn swclkpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Serial Wire Data Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn swdiopen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Serial Wire Viewer Output Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn swopen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:9 - I/O Location" ]
    # [ allow ( dead_code ) ]
    pub fn swlocation(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 8u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 12 - ETM Trace Clock Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn tclkpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - ETM Trace Data Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn td0pen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - ETM Trace Data Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn td1pen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - ETM Trace Data Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn td2pen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - ETM Trace Data Pin Enable" ]
    # [ allow ( dead_code ) ]
    pub fn td3pen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 24:25 - I/O Location" ]
    # [ allow ( dead_code ) ]
    pub fn etmlocation(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 24u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Insense {
    register: ::volatile_register::RW<u32>,
}

impl Insense {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&InsenseR, &'w mut InsenseW) -> &'w mut InsenseW
    {
        let bits = self.register.read();
        let r = InsenseR { bits: bits };
        let mut w = InsenseW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> InsenseR {
        InsenseR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut InsenseW) -> &mut InsenseW
    {
        let mut w = InsenseW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct InsenseR {
    bits: u32,
}

impl InsenseR {
    # [ doc = "Bit 0 - Interrupt Sense Enable" ]
    # [ allow ( dead_code ) ]
    pub fn int(&self) -> bool {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - PRS Sense Enable" ]
    # [ allow ( dead_code ) ]
    pub fn prs(&self) -> bool {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct InsenseW {
    bits: u32,
}

impl InsenseW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        InsenseW { bits: 3 }
    }
    # [ doc = "Bit 0 - Interrupt Sense Enable" ]
    # [ allow ( dead_code ) ]
    pub fn int(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - PRS Sense Enable" ]
    # [ allow ( dead_code ) ]
    pub fn prs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Lock {
    register: ::volatile_register::RW<u32>,
}

impl Lock {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&LockR, &'w mut LockW) -> &'w mut LockW
    {
        let bits = self.register.read();
        let r = LockR { bits: bits };
        let mut w = LockW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> LockR {
        LockR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut LockW) -> &mut LockW
    {
        let mut w = LockW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct LockR {
    bits: u32,
}

impl LockR {
    # [ doc = "Bits 0:15 - Configuration Lock Key" ]
    # [ allow ( dead_code ) ]
    pub fn lockkey(&self) -> u16 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 65535;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct LockW {
    bits: u32,
}

impl LockW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        LockW { bits: 0 }
    }
    # [ doc = "Bits 0:15 - Configuration Lock Key" ]
    # [ allow ( dead_code ) ]
    pub fn lockkey(&mut self, value: u16) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Ctrl {
    register: ::volatile_register::RW<u32>,
}

impl Ctrl {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CtrlR, &'w mut CtrlW) -> &'w mut CtrlW
    {
        let bits = self.register.read();
        let r = CtrlR { bits: bits };
        let mut w = CtrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> CtrlR {
        CtrlR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CtrlW) -> &mut CtrlW
    {
        let mut w = CtrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct CtrlR {
    bits: u32,
}

impl CtrlR {
    # [ doc = "Bit 0 - Enable EM4 retention" ]
    # [ allow ( dead_code ) ]
    pub fn em4ret(&self) -> bool {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct CtrlW {
    bits: u32,
}

impl CtrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CtrlW { bits: 0 }
    }
    # [ doc = "Bit 0 - Enable EM4 retention" ]
    # [ allow ( dead_code ) ]
    pub fn em4ret(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Cmd {
    register: ::volatile_register::WO<u32>,
}

impl Cmd {
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut CmdW) -> &mut CmdW
    {
        let mut w = CmdW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct CmdW {
    bits: u32,
}

impl CmdW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CmdW { bits: 0 }
    }
    # [ doc = "Bit 0 - EM4 Wake-up clear" ]
    # [ allow ( dead_code ) ]
    pub fn em4wuclr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Em4wuen {
    register: ::volatile_register::RW<u32>,
}

impl Em4wuen {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Em4wuenR, &'w mut Em4wuenW) -> &'w mut Em4wuenW
    {
        let bits = self.register.read();
        let r = Em4wuenR { bits: bits };
        let mut w = Em4wuenW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> Em4wuenR {
        Em4wuenR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Em4wuenW) -> &mut Em4wuenW
    {
        let mut w = Em4wuenW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Em4wuenR {
    bits: u32,
}

impl Em4wuenR {
    # [ doc = "Bits 0:5 - EM4 Wake-up enable" ]
    # [ allow ( dead_code ) ]
    pub fn em4wuen(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 63;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Em4wuenW {
    bits: u32,
}

impl Em4wuenW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Em4wuenW { bits: 0 }
    }
    # [ doc = "Bits 0:5 - EM4 Wake-up enable" ]
    # [ allow ( dead_code ) ]
    pub fn em4wuen(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 63;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Em4wupol {
    register: ::volatile_register::RW<u32>,
}

impl Em4wupol {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn modify_bits<F>(&mut self, f: F)
        where F: FnOnce(&mut u32)
    {
        let mut bits = self.register.read();
        f(&mut bits);
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub unsafe fn write_bits(&mut self, bits: u32) {
        self.register.write(bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Em4wupolR, &'w mut Em4wupolW) -> &'w mut Em4wupolW
    {
        let bits = self.register.read();
        let r = Em4wupolR { bits: bits };
        let mut w = Em4wupolW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> Em4wupolR {
        Em4wupolR { bits: self.register.read() }
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Em4wupolW) -> &mut Em4wupolW
    {
        let mut w = Em4wupolW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Em4wupolR {
    bits: u32,
}

impl Em4wupolR {
    # [ doc = "Bits 0:5 - EM4 Wake-up Polarity" ]
    # [ allow ( dead_code ) ]
    pub fn em4wupol(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 63;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Em4wupolW {
    bits: u32,
}

impl Em4wupolW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Em4wupolW { bits: 0 }
    }
    # [ doc = "Bits 0:5 - EM4 Wake-up Polarity" ]
    # [ allow ( dead_code ) ]
    pub fn em4wupol(&mut self, value: u8) -> &mut Self {
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        # [ allow ( dead_code ) ]
        const MASK: u8 = 63;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Em4wucause {
    register: ::volatile_register::RO<u32>,
}

impl Em4wucause {
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read_bits(&self) -> u32 {
        self.register.read()
    }
    # [ allow ( dead_code , missing_docs ) ]
    pub fn read(&self) -> Em4wucauseR {
        Em4wucauseR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
# [ allow ( missing_docs ) ]
pub struct Em4wucauseR {
    bits: u32,
}

impl Em4wucauseR {
    # [ doc = "Bits 0:5 - EM4 wake-up cause" ]
    # [ allow ( dead_code ) ]
    pub fn em4wucause(&self) -> u8 {
        # [ allow ( dead_code ) ]
        const MASK: u32 = 63;
        # [ allow ( dead_code ) ]
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}
