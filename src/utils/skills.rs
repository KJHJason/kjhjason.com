use crate::templates::general::SkillInfo;

#[inline(always)]
pub fn get_languages<'a>() -> Vec<SkillInfo<'a>> {
    vec![
        SkillInfo {
            link: "https://wikipedia.org/wiki/HTML",
            img_src: "https://storage.kjhjason.com/images/skills/html.webp",
            img_alt: "HTML5",
            name: "HTML",
        },
        SkillInfo {
            link: "https://wikipedia.org/wiki/CSS",
            img_src: "https://storage.kjhjason.com/images/skills/css.webp",
            img_alt: "CSS3",
            name: "CSS",
        },
        SkillInfo {
            link: "https://wikipedia.org/wiki/JavaScript",
            img_src: "https://storage.kjhjason.com/images/skills/js.webp",
            img_alt: "JavaScript",
            name: "JavaScript",
        },
        SkillInfo {
            link: "https://www.python.org/",
            img_src: "https://storage.kjhjason.com/images/skills/python.webp",
            img_alt: "Python",
            name: "Python",
        },
        SkillInfo {
            link: "https://en.wikipedia.org/wiki/C%2B%2B",
            img_src: "https://storage.kjhjason.com/images/skills/cpp.webp",
            img_alt: "C++",
            name: "C++",
        },
        SkillInfo {
            link: "https://go.dev/",
            img_src: "https://storage.kjhjason.com/images/skills/go.webp",
            img_alt: "Go",
            name: "Go/Golang",
        },
        SkillInfo {
            link: "https://learn.microsoft.com/en-us/dotnet/csharp/",
            img_src: "https://storage.kjhjason.com/images/skills/csharp.webp",
            img_alt: "C#",
            name: "C#",
        },
        SkillInfo {
            link: "https://www.typescriptlang.org/",
            img_src: "https://storage.kjhjason.com/images/skills/ts.webp",
            img_alt: "TypeScript",
            name: "TypeScript",
        },
        SkillInfo {
            link: "https://www.rust-lang.org/",
            img_src: "https://storage.kjhjason.com/images/skills/rust.webp",
            img_alt: "Rust",
            name: "Rust",
        },
    ]
}

#[inline(always)]
pub fn get_backend<'a>() -> Vec<SkillInfo<'a>> {
    vec![
        SkillInfo {
            link: "https://flask.palletsprojects.com/",
            img_src: "https://storage.kjhjason.com/images/skills/py-flask.webp",
            img_alt: "Flask",
            name: "Flask",
        },
        SkillInfo {
            link: "https://fastapi.tiangolo.com/",
            img_src: "https://storage.kjhjason.com/images/skills/fastapi.webp",
            img_alt: "FastAPI",
            name: "FastAPI",
        },
        SkillInfo {
            link: "https://dotnet.microsoft.com/en-us/",
            img_src: "https://storage.kjhjason.com/images/skills/net-core.webp",
            img_alt: ".NET Core",
            name: ".NET Core",
        },
        SkillInfo {
            link: "https://pkg.go.dev/net/http",
            img_src: "https://storage.kjhjason.com/images/skills/go.webp",
            img_alt: "Go",
            name: "net/http",
        },
        SkillInfo {
            link: "https://actix.rs/",
            img_src: "https://storage.kjhjason.com/images/skills/actix.webp",
            img_alt: "Actix Web",
            name: "Actix Web",
        },
    ]
}

#[inline(always)]
pub fn get_frontend<'a>() -> Vec<SkillInfo<'a>> {
    vec![
        SkillInfo {
            link: "https://getbootstrap.com/",
            img_src: "https://storage.kjhjason.com/images/skills/bootstrap.webp",
            img_alt: "Bootstrap",
            name: "Bootstrap",
        },
        SkillInfo {
            link: "https://jquery.com/",
            img_src: "https://storage.kjhjason.com/images/skills/jquery.webp",
            img_alt: "jQuery",
            name: "jQuery",
        },
        SkillInfo {
            link: "https://jinja.palletsprojects.com/",
            img_src: "https://storage.kjhjason.com/images/skills/jinja.webp",
            img_alt: "Jinja",
            name: "Jinja",
        },
        SkillInfo {
            link: "https://tailwindcss.com/",
            img_src: "https://storage.kjhjason.com/images/skills/tailwindcss.webp",
            img_alt: "TailwindCSS",
            name: "TailwindCSS",
        },
        SkillInfo {
            link: "https://dotnet.microsoft.com/en-us/apps/aspnet/web-apps/blazor",
            img_src: "https://storage.kjhjason.com/images/skills/blazor.webp",
            img_alt: "Blazor",
            name: "Blazor",
        },
        SkillInfo {
            link: "https://svelte.dev/",
            img_src: "https://storage.kjhjason.com/images/skills/svelte.webp",
            img_alt: "Svelte",
            name: "Svelte",
        },
        SkillInfo {
            link: "https://react.dev/",
            img_src: "https://storage.kjhjason.com/images/skills/react.webp",
            img_alt: "React",
            name: "React",
        },
        SkillInfo {
            link: "https://pkg.go.dev/html/template",
            img_src: "https://storage.kjhjason.com/images/skills/go-template.webp",
            img_alt: "Go Template Logo by jinliming2",
            name: "Go Template",
        },
        SkillInfo {
            link: "https://djc.github.io/askama/askama.html",
            img_src: "https://storage.kjhjason.com/images/skills/rust.webp",
            img_alt: "Rust Logo",
            name: "Askama",
        },
        SkillInfo {
            link: "https://htmx.org/",
            img_src: "https://storage.kjhjason.com/images/skills/htmx.webp",
            img_alt: "htmx",
            name: "htmx",
        },
    ]
}

#[inline(always)]
pub fn get_desktop_apps<'a>() -> Vec<SkillInfo<'a>> {
    vec![SkillInfo {
        link: "https://wails.io/",
        img_src: "https://storage.kjhjason.com/images/skills/wails.webp",
        img_alt: "Wails",
        name: "Wails",
    }]
}

#[inline(always)]
pub fn get_database<'a>() -> Vec<SkillInfo<'a>> {
    vec![
        SkillInfo {
            link: "https://www.sqlite.org/",
            img_src: "https://storage.kjhjason.com/images/skills/sqlite.webp",
            img_alt: "SQLite",
            name: "SQLite",
        },
        SkillInfo {
            link: "https://www.mysql.com/",
            img_src: "https://storage.kjhjason.com/images/skills/mysql.webp",
            img_alt: "MySQL",
            name: "MySQL",
        },
        SkillInfo {
            link: "https://www.mongodb.com/",
            img_src: "https://storage.kjhjason.com/images/skills/mongodb.webp",
            img_alt: "MongoDB",
            name: "MongoDB",
        },
        SkillInfo {
            link: "https://www.microsoft.com/sql-server",
            img_src: "https://storage.kjhjason.com/images/skills/ms-sql-server.webp",
            img_alt: "Microsoft SQL Server",
            name: "MS SQL Server",
        },
    ]
}

#[inline(always)]
pub fn get_deployment<'a>() -> Vec<SkillInfo<'a>> {
    vec![
        SkillInfo {
            link: "https://www.docker.com/",
            img_src: "https://storage.kjhjason.com/images/skills/docker.webp",
            img_alt: "Docker",
            name: "Docker",
        },
        SkillInfo {
            link: "https://cloud.google.com/",
            img_src: "https://storage.kjhjason.com/images/skills/gcp.webp",
            img_alt: "Google Cloud Platform",
            name: "Google Cloud",
        },
        SkillInfo {
            link: "https://www.cloudflare.com/",
            img_src: "https://storage.kjhjason.com/images/skills/cloudflare.webp",
            img_alt: "Cloudflare",
            name: "Cloudflare",
        },
        SkillInfo {
            link: "https://www.alibabacloud.com/",
            img_src: "https://storage.kjhjason.com/images/skills/alibaba-cloud.webp",
            img_alt: "Alibaba Cloud",
            name: "Alibaba Cloud",
        },
        SkillInfo {
            link: "https://fly.io/",
            img_src: "https://storage.kjhjason.com/images/skills/flyio.webp",
            img_alt: "Fly.io",
            name: "Fly.io",
        },
    ]
}

#[inline(always)]
pub fn get_general<'a>() -> Vec<SkillInfo<'a>> {
    vec![
        SkillInfo {
            link: "https://git-scm.com/",
            img_src: "https://storage.kjhjason.com/images/skills/git.webp",
            img_alt: "Git",
            name: "Git",
        },
        SkillInfo {
            link: "https://en.wikipedia.org/wiki/Regular_expression",
            img_src: "https://storage.kjhjason.com/images/skills/regex.webp",
            img_alt: "Regular Expression",
            name: "Regex",
        },
        SkillInfo {
            link: "https://en.wikipedia.org/wiki/Markdown",
            img_src: "https://storage.kjhjason.com/images/skills/markdown.webp",
            img_alt: "Markdown",
            name: "Markdown",
        },
        SkillInfo {
            link: "https://www.selenium.dev/",
            img_src: "https://storage.kjhjason.com/images/skills/selenium.webp",
            img_alt: "Selenium",
            name: "Selenium",
        },
        SkillInfo {
            link: "https://en.wikipedia.org/wiki/Bash_(Unix_shell)",
            img_src: "https://storage.kjhjason.com/images/skills/shell.webp",
            img_alt: "Shell Script",
            name: "Shell Script",
        },
        SkillInfo {
            link: "https://microsoft.com/powershell",
            img_src: "https://storage.kjhjason.com/images/skills/powershell.webp",
            img_alt: "PowerShell",
            name: "PowerShell",
        },
        SkillInfo {
            link: "https://en.wikipedia.org/wiki/Microsoft_Windows",
            img_src: "https://storage.kjhjason.com/images/skills/windows.webp",
            img_alt: "Windows",
            name: "Windows",
        },
        SkillInfo {
            link: "https://en.wikipedia.org/wiki/Linux",
            img_src: "https://storage.kjhjason.com/images/skills/linux.webp",
            img_alt: "Linux",
            name: "Linux",
        },
    ]
}
