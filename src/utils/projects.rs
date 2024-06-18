use crate::templates::general::ProjectInfo;

pub fn get_projects<'a>() -> Vec<ProjectInfo<'a>> {
    vec![
        ProjectInfo {
            title: "kjhjason.com",
            img: "/static/images/projects/kjhjason.com.png",
            img_alt: "Screenshot of the Blog Editing page of the kjhjason.com website",
            desc: "This project is the website you are currently viewing. It is a personal project that I have started to showcase my portfolio and blog posts.
            <br /><br />It is developed using Actix Web, a Rust web framework, MongoDB as the database, and Cloudflare R2 for storing images.
            <br /><br />Moroever, it uses Askama for templating which is similar to Jinja2 in Python and TailwindCSS for the frontend.",
            tags: vec!["Web Dev", "Rust", "Actix", "Askama", "TailwindCSS", "MongoDB", "Cloud"],
            link: "https://github.com/KJHJason/kjhjason.com",
            date: "Jun 2024 - Present",
        },
        ProjectInfo {
            title: "hmac-serialiser-rs",
            img: "/static/images/projects/hmac-serialiser-rs.png",
            img_alt: "Screenshot of some of the code in the hmac-serialiser-rs project",
            desc: "Similar to the HMACSerialiser project, this is a Rust implementation as I wanted to create shorter but secure tokens for the website you are currently viewing.",
            tags: vec!["Security", "Cryptography", "Rust"],
            link: "https://github.com/KJHJason/hmac-serialiser-rs",
            date: "Jun 2024 - Jun 2024",
        },
        ProjectInfo {
            title: "HMACSerialiser",
            img: "/static/images/projects/hmac-serialiser.png",
            img_alt: "Screenshot of some of the code in the HMACSerialiser project",
            desc: "This is one of my personal projects that I have started to address the use of SHA256(message | secretKey) in the company
            I was attached to during my internship which is susceptible to length extension attacks.
            <br /><br />This project is heavily inspired by the itsdangerous serialiser used in Python web frameworks like Flask.",
            tags: vec!["Security", "Cryptography", "C#"],
            link: "https://github.com/KJHJason/HMACSerialiser",
            date: "Feb 2024 - May 2024",
        },
        ProjectInfo {
            title: "Cultured Downloader",
            img: "/static/images/projects/cultured-downloader-pixiv.png",
            img_alt: "Screenshot of the Pixiv section of the Cultured Downloader GUI",
            desc: "This is one of my personal projects that I have started. Initially it was a CLI-based program written in Python that allows users to download files
            from various websites like Pixiv via web scraping.
            <br /><br />However, I then decided to re-write the program into a GUI-based program with Golang as the backend and Svelte as the frontend using the Wails framework to enhance the user experience.",
            tags: vec!["GUI", "Golang", "Svelte", "wails"],
            link: "https://github.com/KJHJason/Cultured-Downloader",
            date: "Mar 2022 - Present",
        },
        ProjectInfo {
            title: "Mirai (Infosecurity Project)",
            img: "/static/images/projects/mirai-index.png",
            img_alt: "Screenshot of the index page of the Mirai Web App",
            desc: "The Infosecurity Project module focuses on applying data privacy and adhering to standards such as the Personal Data Protection Act in applications.
            <br /><br />Our team has chosen to develop a Social Media Web Application similar to Twitter or X.
            <br /><br />My role primarily involves developing essential features such as the login feature, chat feature,
            and more with various data security measures which helped me obtain my Distinction for this module.
            <br /><br />Additionally, I assist team members with deployment and address any features that require additional attention.",
            tags: vec!["Web Dev", "Python", "FastAPI", "TailwindCSS", "MongoDB", "Cloud"],
            link: "https://github.com/KJHJason/ISPJ-REVISED",
            date: "Oct 2022 - Nov 2023",
        },
        ProjectInfo {
            title: "CourseFinity (App Security)",
            img: "/static/images/projects/coursefinity-app-sec.png",
            img_alt: "Screenshot of the login page of the CourseFinity Web App",
            desc: "The main focus of the App Security Project is to develop a secure web application while adhering to the OWASP Top 10 standards.
            <br /><br />My main areas for addressing the OWASP Top 10 2021 were Cryptographic Failures and Identification and Authentication Failures.
            <br /><br />This is also my first time touching Cloud Technologies to enhance the security of the Web Application which helped me obtain my Distinction for this module.",
            tags: vec!["Web Dev", "Python", "Flask", "Bootstrap 5", "MySQL", "Cloud"],
            link: "https://github.com/KJHJason/IT2555-Applications-Security-Project",
            date: "Apr 2022 - Aug 2022",
        },
        ProjectInfo {
            title: "Staycation Management System",
            img: "/static/images/projects/nyp-dsa-cli.png",
            img_alt: "Screenshot of the CLI-based program",
            desc: "This assignment is a CLI-based program written in Python which allows a staff to view, search, sort, and delete the booking records.
            <br /><br />However, this assignment is part of the Data Structures and Algorithms module which focuses
            on the implementation of various data structures and algorithms to fulfil the minimum requirements.
            <br /><br />Moreover, I had learnt new data structures and algorithms outside of the module like introsort that helped me obtain my Distinction for this module.",
            tags: vec!["CLI", "DSA", "Python"],
            link: "https://github.com/KJHJason/NYP-DSA-Assignment",
            date: "Apr 2022 - Jun 2022",
        },
        ProjectInfo {
            title: "CourseFinity (App Development)",
            img: "/static/images/projects/coursefinity-app-dev.png",
            img_alt: "Screenshot of the login page of the CourseFinity Web App",
            desc: r#"This is the first time I have developed a dynamic website using Flask and Jinja2. The website concept is similar to Coursera where users can purchase courses and view them.
            <br /><br />Although similar to the project in the App Security Project module, the web applications lack security features as the focus is on developing the website.
            <br /><br />However, I did implement some security features like 2FA and difficult features like the "naive" recommendation algorithm which helped me obtain my Distinction for this module.
            "#,
            tags: vec!["Web Dev", "Python", "Bootstrap 5", "Flask", "Shelve"],
            link: "https://github.com/KJHJason/App-Development-Project",
            date: "Oct 2021 - Feb 2022",
        },
        ProjectInfo {
            title: "GPA Calculator",
            img: "/static/images/projects/gpa-calculator.png",
            img_alt: "Screenshot of the CLI-based program",
            desc: "This project is a CLI-based program written in C++ where users can calculate their GPA based on the modules they have taken.
            <br />It is one of my first personal projects that I have completed outside of my curriculum.",
            tags: vec!["CLI", "C++"],
            link: "https://github.com/KJHJason/GPACalculator",
            date: "Apr 2022 - Apr 2022",
        },
        ProjectInfo {
            title: "OtakuAbroadJapan",
            img: "/static/images/projects/web-development.png",
            img_alt: "Screenshot of the OtakuAbroadJapan Website",
            desc: "Similar to Web Publishing in PFP, students were tasked to create a static website. However, the Web Development module focuses more on JavaScript to enhance the website.",
            tags: vec!["Web Dev", "Bootstrap 5", "JavaScript"],
            link: "https://github.com/KJHJason/OtakuAbroadJapan-Website",
            date: "Aug 2021 - Jan 2022",
        },
        ProjectInfo {
            title: "Vending Machine",
            img: "/static/images/projects/programming-essentials.png",
            img_alt: "Screenshot of the CLI-based vending machine program",
            desc: "Part of the Programming Essentials module, this project consists of students creating a CLI-based vending machine program in Python with various requirements to fulfil.
            <br /><br />This is the first time I have worked with any programming languages so it was challenging to write code due to the unfamiliar syntax.
            <br /><br />Additionally, understanding concepts of deep copy, shallow copy, and more took time to grasp.",
            tags: vec!["Introductory", "CLI", "Python"],
            link: "https://github.com/KJHJason/Vending-Machine",
            date: "June 2021 - July 2021",
        },
        ProjectInfo {
            title: "JJS Tuition Centre",
            img: "/static/images/projects/web-publishing.png",
            img_alt: "Screenshot of JJS Tuition Centre Website",
            desc: "The first static website I created for the Web Publishing module that introduced me to HTML, CSS, and Bootstrap.",
            tags: vec!["Introductory", "PFP", "Web Dev", "Bootstrap 3"],
            link: "https://github.com/KJHJason/JJS-Tuition-Centre-Website",
            date: "July 2020 - Oct 2020",
        },
        ProjectInfo {
            title: "IT Application Project",
            img: "/static/images/projects/it-application-project.png",
            img_alt: "Screenshot of the Cozmo Block Programming",
            desc: "A simple module that introduced block programming to me using a Cozmo toy robot to do simple tasks.",
            tags: vec!["Introductory", "PFP"],
            link: "",
            date: "Dec 2020 - Feb 2021",
        },
    ]
}
