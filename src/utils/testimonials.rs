use crate::templates::general::TestimonialInfo;

#[inline(always)]
pub fn get_testimonials<'a>() -> Vec<TestimonialInfo<'a>> {
    vec![
        TestimonialInfo {
            issuer: "RSM Stone Forest",
            file_url: "https://drive.proton.me/urls/1KVP5C0014#h7BRNh05sHV6",
            date: "Jan 2024",
            img_src: "https://storage.kjhjason.com/images/testimonials/rsm-stone-forest.webp",
            img_alt: "RSM Stone Forest logo",
        },
        TestimonialInfo {
            issuer: "Bendemeer Secondary School",
            file_url: "https://drive.proton.me/urls/9BHBM4JE3W#Bo3UtXDwHhsl",
            date: "2020",
            img_src: "https://storage.kjhjason.com/images/testimonials/bendemeer-secondary.webp",
            img_alt: "Bendemeer Secondary School logo",
        },
    ]
}
