use crate::templates::general::ExperienceInfo;

#[inline(always)]
pub fn get_experiences<'a>() -> Vec<ExperienceInfo<'a>> {
    vec![
        ExperienceInfo {
            time: "Jan 2010 - Dec 2015",
            finished: true,
            title: "Hong Wen School",
            sub_title: "Primary School Leaving Examination",
            desc: "I graduated from Hong Wen School with a Primary School Leaving Examination (PSLE) score of 155 and was a member of the Basketball Club. My score was not very high, qualifying me only for the Normal Technical and Normal Academic streams in secondary school.",
        },
        ExperienceInfo {
            time: "Jan 2016 - Jan 2020",
            finished: true,
            title: "Bendemeer Secondary School",
            sub_title: "GCE N(A)-Level + GCE O-Level Mathematics",
            desc: "I graduated from Bendemeer Secondary School with an English, Maths, and Best 3 subjects (ELMAB3) score of 7 for my GCE N(A)-Level and a B3 for my GCE O-Level Mathematics. Additionally, I was an Executive Committee Member of the Infocomm Club and created a video montage for the school song, which was ultimately chosen by the Club Advisor.",
        },
        ExperienceInfo {
            time: "Nov 2019 - Dec 2019",
            finished: true,
            title: "Quantium Solutions",
            sub_title: "Logistic Assistant (Part-time)",
            desc: "As a logistics assistant, my job involves packing orders from Beautiful.Me for shipping, recording packed orders for the shipping manifest in Excel sheets, resolving any duplicate and missing parcels, and clearing the waste area by segregating cardboard and plastic waste for recycling.",
        },
        ExperienceInfo {
            time: "Apr 2020 - Apr 2021",
            finished: true,
            title: "Nanyang Polytechnic",
            sub_title: "Polytechnic Foundation Programme",
            desc: "I completed the Polytechnic Foundation Programme (PFP) at Nanyang Polytechnic with a cGPA of 3.4. During this one-year programme, I studied introductory modules such as Web Publishing before continuing on to a three-year tenure at Nanyang Polytechnic to obtain my Diploma in Cybersecurity & Digital Forensics. Additionally, I was a member of the Sakuran Japanese Cultural Club (M.A.I.D), where I learned how to draw Japanese anime art traditionally.",
        },
        ExperienceInfo {
            time: "Apr 2021 - Apr 2024",
            finished: true,
            title: "Nanyang Polytechnic",
            sub_title: "Diploma in Cybersecurity & Digital Forensics",
            desc: "I graduated from Nanyang Polytechnic with a Diploma in Cybersecurity & Digital Forensics, achieving a cGPA of 3.85 and earning 9 CCA Points. Moreover, I consistently made it to the Director's List for 4 out of 6 semesters and received multiple Distinctions in various modules. Additionally, I completed Diploma+ modules in Linux Foundation and Cloud Computing Foundation, and participated in notable events such as virtual exchanges with Tama University and others.",
        },
        ExperienceInfo {
            time: "Sep 2023 - Feb 2024",
            finished: true,
            title: "RSM Stone Forest",
            sub_title: "Software Engineer (Internship)",
            desc: "During my internship at RSM Stone Forest Singapore, I contributed to developing BankLink and the Document Depository project. In BankLink, I focused on database design, translated transaction details from VB.NET to C#, and implemented features such as GST Configuration and Subscription/Plan management. For the Document Depository, I researched secure PDF preview methods. I also explored GitHub Copilot and presented my findings internally. Nominated for distinction, this experience enhanced my skills in C#, ASP.NET, database design, frontend development, research, and security analysis.",
        },
        ExperienceInfo {
            time: "TBD - 2026/2027",
            finished: false,
            title: "National Service",
            sub_title: "",
            desc: "",
        },
        ExperienceInfo {
            time: "2026/2027 - TBD",
            finished: false,
            title: "Singapore Management University",
            sub_title: "Bachelor of Science (Computer Science)",
            desc: "",
        },
    ]
}
