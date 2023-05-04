// Global Descriptor Table(GDT) 초기화
pub fn init() {
    use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
    use x86_64::structures::gdt::SegmentSelector;

    // GDT를 생성합니다.
    let mut gdt = GlobalDescriptorTable::new();

    // 코드 세그먼트를 생성합니다.
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());

    // 데이터 세그먼트를 생성합니다.
    let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());

    // TSS(태스크 상태 세그먼트)를 생성합니다.
    // TSS는 태스크 스위칭에 사용됩니다.
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));

    // GDT를 로드합니다.
    // CPU는 code_selector와 data_selector를 사용하여 코드와 데이터에 액세스할 수 있습니다.
    // tss_selector는 태스크 스위칭에 사용됩니다.
    let code_selector = SegmentSelector::from_raw(code_selector as u16);
    let data_selector = SegmentSelector::from_raw(data_selector as u16);
    let tss_selector = SegmentSelector::from_raw(tss_selector as u16);
    gdt.load();
}
