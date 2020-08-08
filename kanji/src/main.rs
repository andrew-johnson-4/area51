fn main() {
   assert!(!kanji::is_kanji(&'あ'));
   assert!(kanji::is_kanji(&'字'));
   assert!(kanji::is_kanji(&'获'));
}
