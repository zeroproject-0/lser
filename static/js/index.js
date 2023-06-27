const $form = document.getElementById('form');
const $searchInput = document.getElementById('query');

$form.addEventListener('submit', (e) => {
  e.preventDefault();
  console.log('Submit', $searchInput.value);
})
