// 인터럽트 초기화
pub fn init() {
    // PIC(Programmable Interrupt Controller)를 초기화합니다.
    // PIC는 8259A 칩셋으로 CPU에 인터럽트를 전달합니다.
    unsafe {
        pic::init();
    }

    // APIC(Advanced Programmable Interrupt Controller)를 초기화합니다.
    // APIC는 로컬 인터럽트 컨트롤러입니다.
    unsafe {
        apic::init();
    }

    // 인터럽트 핸들러를 초기화합니다.
    IDT.load();
}
