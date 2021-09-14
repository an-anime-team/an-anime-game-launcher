// When page is fully loaded
window.addEventListener('DOMContentLoaded', () => {
    // Change the text inside tags with greeting class
    document.querySelector('.greeting').innerText = 'Hello World';
}, false);