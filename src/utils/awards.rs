use crate::templates::general::AwardInfo;

#[inline(always)]
pub fn get_awards<'a>() -> Vec<AwardInfo<'a>> {
    vec![
        AwardInfo{
            title: "Director's List for Year 3 Semester 2",
            issuer: "Nanyang Polytechnic",
            file_url: "https://drive.proton.me/urls/G2FC3XFW1W#TqIGyMLhGazS",
            date: "May 2024",
            img_src: "https://storage.kjhjason.com/images/awards/Y3_S2_201484K_KUAN JUN HAO JASON-1.webp",
            img_alt: "Year 3 Semester 2 Director's List Award Certificate",
        },
        AwardInfo{
            title: "Director's List for Year 3 Semester 1",
            issuer: "Nanyang Polytechnic",
            file_url: "https://drive.proton.me/urls/4HHNTA6QW8#LjWBNGMpLLRD",
            date: "Jan 2024",
            img_src: "https://storage.kjhjason.com/images/awards/Y3_S1_201484K_KUAN JUN HAO JASON-1.webp",
            img_alt: "Year 3 Semester 1 Director's List Award Certificate",
        },
        AwardInfo{
            title: "Director's List for Year 2 Semester 2",
            issuer: "Nanyang Polytechnic",
            file_url: "https://drive.proton.me/urls/4HHNTA6QW8#LjWBNGMpLLRD",
            date: "May 2023",
            img_src: "https://storage.kjhjason.com/images/awards/Y2_S2_SF2102_KUAN JUN HAO JASON_2022_S2-1.webp",
            img_alt: "Year 2 Semester 2 Director's List Award Certificate",
        },
        AwardInfo{
            title: "Edusave Certificate of Academic Achievement 2023",
            issuer: "Ministry of Education",
            file_url: "https://storage.kjhjason.com/images/awards/edusave_cert_of_academic_achievement_2023.webp",
            date: "2023",
            img_src: "https://storage.kjhjason.com/images/awards/edusave_cert_of_academic_achievement_2023.webp",
            img_alt: "Edusave Certificate of Academic Achievement 2023 Award Certificate",
        },
        AwardInfo{
            title: "Edusave Merit Bursary 2023",
            issuer: "Ministry of Education",
            file_url: "https://storage.kjhjason.com/images/awards/edusave_merit_bursary_2023.webp",
            date: "2023",
            img_src: "https://storage.kjhjason.com/images/awards/edusave_merit_bursary_2023.webp",
            img_alt: "Edusave Merit Bursary 2023 Award Certificate",
        },
        AwardInfo{
            title: "Director's List for Year 1 Semester 2",
            issuer: "Nanyang Polytechnic",
            file_url: "https://drive.proton.me/urls/4HHNTA6QW8#LjWBNGMpLLRD",
            date: "Jun 2022",
            img_src: "https://storage.kjhjason.com/images/awards/Y1_S2_ITDF13_2021S2_DirList1335-1.webp",
            img_alt: "Year 1 Semester 2 Director's List Award Certificate",
        },
        AwardInfo{
            title: "Edusave Merit Bursary 2022",
            issuer: "Ministry of Education",
            file_url: "https://storage.kjhjason.com/images/awards/edusave_merit_bursary_2022.webp",
            date: "2022",
            img_src: "https://storage.kjhjason.com/images/awards/edusave_merit_bursary_2022.webp",
            img_alt: "Edusave Merit Bursary 2022 Award Certificate",
        },
        AwardInfo{
            title: "Edusave Certificate of Academic Achievement 2022",
            issuer: "Ministry of Education",
            file_url: "https://storage.kjhjason.com/images/awards/edusave_cert_of_academic_achievement_2022.webp",
            date: "2022",
            img_src: "https://storage.kjhjason.com/images/awards/edusave_cert_of_academic_achievement_2022.webp",
            img_alt: "Edusave Certificate of Academic Achievement 2022 Award Certificate",
        },
        AwardInfo{
            title: "Edusave Merit Bursary 2021",
            issuer: "Ministry of Education",
            file_url: "https://storage.kjhjason.com/images/awards/edusave_merit_bursary_2021.webp",
            date: "2021",
            img_src: "https://storage.kjhjason.com/images/awards/edusave_merit_bursary_2021.webp",
            img_alt: "Edusave Merit Bursary 2021 Award Certificate",
        },
        AwardInfo{
            title: "Edusave Certificate of Academic Achievement 2021",
            issuer: "Ministry of Education",
            file_url: "https://storage.kjhjason.com/images/awards/edusave_cert_of_academic_achievement_2021.webp",
            date: "2021",
            img_src: "https://storage.kjhjason.com/images/awards/edusave_cert_of_academic_achievement_2021.webp",
            img_alt: "Edusave Certificate of Academic Achievement 2021 Award Certificate",
        },
        AwardInfo{
            title: "Oustanding Performance in GCE 'N' Level Examination (NA) 2019",
            issuer: "Bendemeer Secondary School",
            file_url: "https://storage.kjhjason.com/images/awards/bendemeer_sci_human_dnt_2020.webp",
            date: "Jun 2020",
            img_src: "https://storage.kjhjason.com/images/awards/bendemeer_sci_human_dnt_2020.webp",
            img_alt: "Oustanding Performance in GCE 'N' Level Examination (NA) 2019 Award Certificate",
        },
        AwardInfo{
            title: "Edusave Scholarship 2019",
            issuer: "Ministry of Education",
            file_url: "https://storage.kjhjason.com/images/awards/edusave_scholarship_2019.webp",
            date: "2019",
            img_src: "https://storage.kjhjason.com/images/awards/edusave_scholarship_2019.webp",
            img_alt: "Edusave Scholarship 2019 Award Certificate",
        },
        AwardInfo{
            title: "Outstanding Performance - 2nd in Standard",
            issuer: "Bendemeer Secondary School",
            file_url: "https://storage.kjhjason.com/images/awards/bendemeer_second_in_na_2019.webp",
            date: "Apr 2019",
            img_src: "https://storage.kjhjason.com/images/awards/bendemeer_second_in_na_2019.webp",
            img_alt: "Outstanding Performance in Secondary 3 NA 2018 - 2nd in Standard Award Certificate",
        },
        AwardInfo{
            title: "Outstanding Performance - Science and Design & Technology",
            issuer: "Bendemeer Secondary School",
            file_url: "https://storage.kjhjason.com/images/awards/bendemeer_sci_dnt_2019.webp",
            date: "Apr 2019",
            img_src: "https://storage.kjhjason.com/images/awards/bendemeer_sci_dnt_2019.webp",
            img_alt: "Outstanding Performance in Secondary 3 NA 2018 - Science and Design & Technology Award Certificate",
        },
        AwardInfo{
            title: "Edusave Scholarship 2018",
            issuer: "Ministry of Education",
            file_url: "https://storage.kjhjason.com/images/awards/edusave_scholarship_2018.webp",
            date: "2018",
            img_src: "https://storage.kjhjason.com/images/awards/edusave_scholarship_2018.webp",
            img_alt: "Edusave Scholarship 2018 Award Certificate",
        },
        AwardInfo{
            title: "Outstanding Performance - 1st in Standard",
            issuer: "Bendemeer Secondary School",
            file_url: "https://storage.kjhjason.com/images/awards/bendemeer_first_in_na_2018.webp",
            date: "Apr 2018",
            img_src: "https://storage.kjhjason.com/images/awards/bendemeer_first_in_na_2018.webp",
            img_alt: "Outstanding Performance in Secondary 2 NA 2017 - 1st in Standard Award Certificate",
        },
        AwardInfo{
            title: "Outstanding Performance - Science, History & Geography",
            issuer: "Bendemeer Secondary School",
            file_url: "https://storage.kjhjason.com/images/awards/bendemeer_sci_hist_geo_2018.webp",
            date: "Apr 2018",
            img_src: "https://storage.kjhjason.com/images/awards/bendemeer_sci_hist_geo_2018.webp",
            img_alt: "Outstanding Performance in Secondary 2 NA 2017 - Science, History & Geography Award Certificate",
        },
        AwardInfo{
            title: "Edusave Scholarship 2017",
            issuer: "Ministry of Education",
            file_url: "https://storage.kjhjason.com/images/awards/edusave_scholarship_2017.webp",
            date: "2017",
            img_src: "https://storage.kjhjason.com/images/awards/edusave_scholarship_2017.webp",
            img_alt: "Edusave Scholarship 2017 Award Certificate",
        },
        AwardInfo{
            title: "Edusave Certificate of Academic Achievement 2016",
            issuer: "Ministry of Education",
            file_url: "https://storage.kjhjason.com/images/awards/edusave_cert_of_academic_achievement_2016.webp",
            date: "2016",
            img_src: "https://storage.kjhjason.com/images/awards/edusave_cert_of_academic_achievement_2016.webp",
            img_alt: "Edusave Certificate of Academic Achievement 2016 Award Certificate",
        },
        AwardInfo{
            title: "Edusave Merit Bursary 2016",
            issuer: "Ministry of Education",
            file_url: "https://storage.kjhjason.com/images/awards/edusave_merit_award_2016.webp",
            date: "2016",
            img_src: "https://storage.kjhjason.com/images/awards/edusave_merit_award_2016.webp",
            img_alt: "Edusave Merit Bursary 2016 Award Certificate",
        },
    ]
}