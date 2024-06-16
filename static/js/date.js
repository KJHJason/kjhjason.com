const parseDateToLocal = (date, withTime = false) => {
    const dateObj = new Date(date);
    return dateObj.toLocaleDateString("en-US", {
        month: "long", 
        day: "numeric", 
        year: "numeric",
        hour: withTime ? "numeric" : undefined,
        minute: withTime ? "numeric" : undefined,
        second: withTime ? "numeric" : undefined,
        hour12: true
    });
};
