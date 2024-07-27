/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["*.html", "./src/**/*.rs",],
    theme: {
        extend: {
            fontFamily: {
                "lato": ["lato"],
            }
        },
    },
    plugins: [],
}