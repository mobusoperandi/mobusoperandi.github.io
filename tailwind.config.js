module.exports = {
    theme: {
        extend: {
            screens: {
                'short': { 'raw': '(max-height: 500px) and (min-width: 500px)' },
                // => @media (min-height: 800px) { ... }
            }
        }
    }
}
