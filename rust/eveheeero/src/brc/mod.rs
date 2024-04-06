mod bag;
mod file;
use std::num::NonZeroUsize;

// abcd;123
// 이름 오름차순 정렬
// `지역명=최솟값;최댓값;평균값(총합/개수)`
#[inline(always)]
pub(super) unsafe fn solution(path: &str) -> String {
    let section_count: NonZeroUsize = std::thread::available_parallelism().unwrap();
    let file = file::File::new(path);

    std::mem::forget(file);
    "".to_owned()
}

#[inline(always)]
fn hash_string(s: &[u8]) -> u64 {
    let mut hash = 0u64;
    for c in s {
        if c == &0 {
            break;
        }
        hash = hash.wrapping_mul(33).wrapping_add(*c as u64);
    }
    hash
}
