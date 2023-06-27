const $form = document.getElementById('form');
const $searchInput = document.getElementById('query');
const $results = document.getElementById('results');

$form.addEventListener('submit', async (e) => {
  e.preventDefault();
  const query = $searchInput.value;

  try {
    const res = await fetch('search', {
      headers: {
        'Content-Type': 'text/plain'
      },
      method: 'POST',
      body: query,
    });

    const files = await res.json();

    $results.innerHTML = "";
    files.forEach(create_result_card);
  } catch (e) {
    console.error(e);
  }
});

function create_result_card({ doc }) {
  const div = document.createElement('div');
  div.classList.add('results__item');
  const a = document.createElement('a');
  a.classList.add('results__link');
  a.setAttribute('href', `file/${doc}`);
  a.setAttribute('target', '_blank');
  a.textContent = doc.split('/').at(-1);
  div.appendChild(a);
  $results.appendChild(div);
}
