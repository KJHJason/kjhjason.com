use crate::templates::general::ProjectInfo;

#[inline(always)]
pub fn get_projects<'a>() -> Vec<ProjectInfo<'a>> {
    vec![
        ProjectInfo {
            title: "kjhjason.com",
            img: "https://storage.kjhjason.com/images/projects/kjhjason.com.webp",
            img_alt: "Screenshot of the Blog Editing page of the kjhjason.com website",
            desc: r#"This project is the website you are currently viewing that is deployed on <a href="https://fly.io/" target="_blank">Fly.io</a>.
            It is a personal project that I have started to showcase my portfolio and blog posts.
            <br /><br />It is developed using Actix Web, a Rust web framework, MongoDB as the database, and Cloudflare R2 for storing images.
            <br /><br />Moroever, it uses Askama for templating which is similar to Jinja2 in Python and TailwindCSS for the frontend."#,
            tags: vec!["Web Dev", "Rust", "Actix", "Askama", "TailwindCSS", "MongoDB", "Cloud"],
            link: "https://github.com/KJHJason/kjhjason.com",
            presentation_link: "",
            date: "Jun 2024 - Present",
        },
        ProjectInfo {
            title: "hmac-serialiser (Rust)",
            img: "https://storage.kjhjason.com/images/projects/hmac-serialiser-rs.webp",
            img_alt: "Screenshot of some of the code in the hmac-serialiser-rs project",
            desc: r#"Similar to the HMACSerialiser project, this is a Rust implementation as I wanted to create shorter but secure tokens for the website you are currently viewing.
            <br /><br />Note: The cryptographic implementations used are from the <a href="https://github.com/RustCrypto" target="_blank">RustCrypto</a> libraries."#,
            tags: vec!["Security", "Cryptography", "Rust"],
            link: "https://github.com/KJHJason/hmac-serialiser/tree/master/rust",
            presentation_link: "",
            date: "Jun 2024 - Present",
        },
        ProjectInfo {
            title: "hmac-serialiser (C#)",
            img: "https://storage.kjhjason.com/images/projects/hmac-serialiser.webp",
            img_alt: "Screenshot of some of the code in the HMACSerialiser project",
            desc: "This is one of my personal projects that I have started to address the use of SHA256(message | secretKey) in the company
            I was attached to during my internship which is susceptible to length extension attacks.
            <br /><br />This project is heavily inspired by the itsdangerous serialiser used in Python web frameworks like Flask.",
            tags: vec!["Security", "Cryptography", "C#"],
            link: "https://github.com/KJHJason/hmac-serialiser/tree/master/csharp",
            presentation_link: "",
            date: "Feb 2024 - Present",
        },
        ProjectInfo {
            title: "Cultured Downloader",
            img: "https://storage.kjhjason.com/images/projects/cultured-downloader-pixiv.webp",
            img_alt: "Screenshot of the Pixiv section of the Cultured Downloader GUI",
            desc: "This is one of my personal projects that I have started. Initially it was a CLI-based program written in Python that allows users to download files
            from various websites like Pixiv via web scraping.
            <br /><br />However, I then decided to re-write the program into a GUI-based program with Golang as the backend and Svelte as the frontend using the Wails framework to enhance the user experience.",
            tags: vec!["GUI", "Golang", "Svelte", "wails"],
            link: "https://github.com/KJHJason/Cultured-Downloader",
            presentation_link: "",
            date: "Mar 2022 - Present",
        },
        ProjectInfo {
            title: "Cybersecurity Project",
            img: "https://storage.kjhjason.com/images/projects/cspj-implementation-plan.webp",
            img_alt: "Screenshot of the implementation plan for my Cybersecurity Project",
            desc: r#"The Cybersecurity Project module focuses on applying what we have learnt in the course to mitigate various threats like malware attacks in simulated real-world scenarios, such as the SingHealth data breach in 2018.
            <br /><br />My team and I have decided to mitigate the various threats found in the Australian National University (ANU) Data Breach in 2018.
            <br /><br />My tasks were addressing Malware Attacks, Password Attacks, Social Engineering Attacks, and Network Attacks mainly using <a href="https://opnsense.org/" target="_blank">OPNsense</a>."#,
            tags: vec!["Cybersecurity", "Infrastructure", "OPNsense", "VMware"],
            link: "",
            presentation_link: "https://www.canva.com/design/DAGJJIsOSVI/iCc2utLHLcxI3-xyUVv9lA/edit?utm_content=DAGJJIsOSVI&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton",
            date: "May 2023 - Aug 2023",
        },
        ProjectInfo {
            title: "Cyber Forensic Process Assignment",
            img: "https://storage.kjhjason.com/images/projects/cforp.webp",
            img_alt: "Screenshot of one of the pages of my Cyber Forensic Process Assignment",
            desc: r#"Based on the given scenario found in the <a href="https://drive.proton.me/urls/C71J76B3T4#d1LgRgJ1ApL4" target="_blank">assignment brief</a>, I am tasked with applying the steps in preliminary planning, equipment seizure, evidence collection, recording, and safeguarding processes by developing a forensic investigation, as well as serving as an Expert Witness."#,
            tags: vec!["Cybersecurity", "Forensics", "EnCase"],
            link: "",
            presentation_link: "https://drive.proton.me/urls/NQY33N8VC4#sAHxl7xBOVYW",
            date: "Jun 2023 - Jul 2023",
        },
        ProjectInfo {
            title: "Mirai (Infosecurity Project)",
            img: "https://storage.kjhjason.com/images/projects/mirai-index.webp",
            img_alt: "Screenshot of the index page of the Mirai Web App",
            desc: r#"The Infosecurity Project module focuses on applying data privacy and adhering to standards such as the Personal Data Protection Act in applications.
            <br /><br />Our team has chosen to develop a Social Media Web Application similar to Twitter or X.
            <br /><br />My role primarily involves developing essential features such as the login feature, chat feature,
            and more with various data security measures which helped me obtain my Distinction for this module.
            <br /><br />Additionally, I assist team members with deployment and address any features that require additional attention.
            <br /><br />The website has been archived on <a href="https://web.archive.org/web/20230228052024/https://www.miraisocial.live/" target="_blank">Wayback Machine - Internet Archive</a>"#,
            tags: vec!["Web Dev", "Python", "FastAPI", "TailwindCSS", "MongoDB", "Cloud"],
            link: "https://github.com/KJHJason/ISPJ-REVISED",
            presentation_link: "https://www.canva.com/design/DAFxbixHp3M/pmVo_rQHQ8xpFkC6ekmvig/edit?utm_content=DAFxbixHp3M&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton",
            date: "Oct 2022 - Nov 2023",
        },
        ProjectInfo {
            title: "CourseFinity (App Security)",
            img: "https://storage.kjhjason.com/images/projects/coursefinity-app-sec.webp",
            img_alt: "Screenshot of the login page of the CourseFinity Web App",
            desc: r#"The main focus of the App Security Project is to develop a secure web application while adhering to the OWASP Top 10 standards.
            <br /><br />My main areas for addressing the OWASP Top 10 2021 were Cryptographic Failures and Identification and Authentication Failures.
            <br /><br />This is also my first time touching Cloud Technologies to enhance the security of the Web Application which helped me obtain my Distinction for this module.
            <br /><br />The website has been archived on <a href="https://web.archive.org/web/20220819100528/https://coursefinity.social/" target="_blank">Wayback Machine - Internet Archive</a>"#,
            tags: vec!["Web Dev", "Python", "Flask", "Bootstrap 5", "MySQL", "Cloud"],
            link: "https://github.com/KJHJason/IT2555-Applications-Security-Project",
            presentation_link: "https://docs.google.com/presentation/d/1dhIkai6SfHuYox7r67zSZPOn9eqqSkxMTkydQKC97P4/edit?usp=sharing",
            date: "Apr 2022 - Aug 2022",
        },
        ProjectInfo {
            title: "Staycation Management System",
            img: "https://storage.kjhjason.com/images/projects/nyp-dsa-cli.webp",
            img_alt: "Screenshot of the CLI-based program",
            desc: "This assignment is a CLI-based program written in Python which allows a staff to view, search, sort, and delete the booking records.
            <br /><br />However, this assignment is part of the Data Structures and Algorithms module which focuses
            on the implementation of various data structures and algorithms to fulfil the minimum requirements.
            <br /><br />Moreover, I had learnt new data structures and algorithms outside of the module like introsort that helped me obtain my Distinction for this module.",
            tags: vec!["CLI", "DSA", "Python"],
            link: "https://github.com/KJHJason/NYP-DSA-Assignment",
            presentation_link: "https://drive.proton.me/urls/W37KAZ700R#QhRixAQJHC7J",
            date: "Apr 2022 - Jun 2022",
        },
        ProjectInfo {
            title: "CourseFinity (App Development)",
            img: "https://storage.kjhjason.com/images/projects/coursefinity-app-dev.webp",
            img_alt: "Screenshot of the login page of the CourseFinity Web App",
            desc: r#"This is the first time I have developed a dynamic website using Flask and Jinja2. The website concept is similar to Coursera where users can purchase courses and view them.
            <br /><br />Although similar to the project in the App Security Project module, the web applications lack security features as the focus is on developing the website.
            <br /><br />However, I did implement some security features like 2FA and difficult features like the "naive" recommendation algorithm which helped me obtain my Distinction for this module.
            "#,
            tags: vec!["Web Dev", "Python", "Bootstrap 5", "Flask", "Shelve"],
            link: "https://github.com/KJHJason/App-Development-Project",
            presentation_link: "https://drive.proton.me/urls/HAMQBETX38#08TWf4Dc7JUn",
            date: "Oct 2021 - Feb 2022",
        },
        ProjectInfo {
            title: "GPA Calculator",
            img: "https://storage.kjhjason.com/images/projects/gpa-calculator.webp",
            img_alt: "Screenshot of the CLI-based program",
            desc: "This project is a CLI-based program written in C++ where users can calculate their GPA based on the modules they have taken.
            <br />It is one of my first personal projects that I have completed outside of my curriculum.",
            tags: vec!["CLI", "C++"],
            link: "https://github.com/KJHJason/GPACalculator",
            presentation_link: "",
            date: "Apr 2022 - Apr 2022",
        },
        ProjectInfo {
            title: "OtakuAbroadJapan",
            img: "https://storage.kjhjason.com/images/projects/web-development.webp",
            img_alt: "Screenshot of the OtakuAbroadJapan Website",
            desc: r#"Similar to Web Publishing in PFP, students were tasked to create a static website. However, the Web Development module focuses more on JavaScript to enhance the website.
            <br /><br />
            The website has been archived on <a href="https://web.archive.org/web/20210923123819/https://www.otakuabroadjapan.com/" target="_blank">Wayback Machine - Internet Archive</a>"#,
            tags: vec!["Web Dev", "Bootstrap 5", "JavaScript"],
            link: "https://github.com/KJHJason/OtakuAbroadJapan-Website",
            presentation_link: "https://drive.proton.me/urls/MTHCGZAE0R#EEtEYE6g7qQW",
            date: "Aug 2021 - Jan 2022",
        },
        ProjectInfo {
            title: "Vending Machine",
            img: "https://storage.kjhjason.com/images/projects/programming-essentials.webp",
            img_alt: "Screenshot of the CLI-based vending machine program",
            desc: "Part of the Programming Essentials module, this project consists of students creating a CLI-based vending machine program in Python with various requirements to fulfil.
            <br /><br />This is the first time I have worked with any programming languages so it was challenging to write code due to the unfamiliar syntax.
            <br /><br />Additionally, understanding concepts like deep copy, shallow copy, and more took time to grasp.",
            tags: vec!["Introductory", "CLI", "Python"],
            link: "https://github.com/KJHJason/Vending-Machine",
            presentation_link: "",
            date: "June 2021 - July 2021",
        },
        ProjectInfo {
            title: "JJS Tuition Centre",
            img: "https://storage.kjhjason.com/images/projects/web-publishing.webp",
            img_alt: "Screenshot of JJS Tuition Centre Website",
            desc: "The first static website I created for the Web Publishing module that introduced me to HTML, CSS, and Bootstrap.",
            tags: vec!["Introductory", "PFP", "Web Dev", "Bootstrap 3"],
            link: "https://github.com/KJHJason/JJS-Tuition-Centre-Website",
            presentation_link: "https://drive.proton.me/urls/3BTGCZNH78#ovAibuG5jytj",
            date: "July 2020 - Oct 2020",
        },
        ProjectInfo {
            title: "IT Application Project",
            img: "https://storage.kjhjason.com/images/projects/it-application-project.webp",
            img_alt: "Screenshot of the Cozmo Block Programming",
            desc: "A simple module that introduced block programming to me using a Cozmo toy robot to do simple tasks.",
            tags: vec!["Introductory", "PFP"],
            link: "",
            presentation_link: "https://drive.proton.me/urls/GRA3B9XD6M#MurHS0pMVSLf",
            date: "Dec 2020 - Feb 2021",
        },
    ]
}
