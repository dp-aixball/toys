use cpp::cpp;
use crate::LongText;

cpp! {
LongText::LongText() {
        this->internal =
            rust!(LongText_constructor [] -> *mut LongText as "void *" {
                let b = Box::new(LongText::default());
                Box::into_raw(b)
            });
    }
    LongText::~LongText() {
        rust!(LongText_destructor [internal: *mut LongText as "void *"] {
            let _b = unsafe {
                Box::from_raw(internal)
            };
        });
    }
LongText::bytes_count() {
return rust!(LongText_bytes_count [
            internal: &mut LongText as "void *"
        ] -> usize as "size_t" {
            internal.bytes_count()
        });
}
}