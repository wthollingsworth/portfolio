const storedColorscheme = localStorage.getItem('colorscheme');
const initialColorscheme = ['nord', 'gruvbox'].indexOf(storedColorscheme) >= 0
                         ? storedColorscheme
                         : 'nord';
const html = document.querySelector('html');
html.classList.add(initialColorscheme);

const nordButton = document.querySelector('#nord');
nordButton.addEventListener('click', () => {
    localStorage.setItem('colorscheme', 'nord');
    html.classList.add('nord');
    html.classList.remove('gruvbox');
} );

const gruvboxButton = document.querySelector('#gruvbox');
gruvboxButton.addEventListener('click', () => {
    localStorage.setItem('colorscheme', 'gruvbox');
    html.classList.add('gruvbox');
    html.classList.remove('nord');
} )
