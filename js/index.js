const rust = import("../pkg/index.js");

const defaultHtml = `
<div class="a">
  <div class="b">
  My Text
  </div>
</div>
      `;

//language=css
const defaultCss = `
 * { display: block; padding: 12px; }
.a { background: #ff0000; }
.b { background: #ffa500; }
.c { background: #ffff00; }
.d { background: #008000; }
.e { background: #0000ff; }
.f { background: #4b0082; }
.g { background: #800080; }
    `;

let renderer;

window.onload = () => {
  const htmlBox = document.getElementById("htmlcode");
  const cssBox = document.getElementById("csscode");
  const displayListBox = document.getElementById("display-list");
  const styleListBox = document.getElementById("style-list");
  const domListBox = document.getElementById("dom-list");
  const canvasWrapper = document.getElementById("canvas");

  htmlBox.value = defaultHtml.trim();
  cssBox.value = defaultCss.trim();

  const updateBox = (node, value, level) => {
    node.innerHTML = "";
    node.appendChild(renderjson.set_show_to_level(level)(value));
  };

  const render = () => {
    if (!renderer) return;
    const res = renderer.render(htmlBox.value, cssBox.value);
    // console.log( Prism.languages)
    updateBox(displayListBox, res.display_list, 3);
    updateBox(domListBox, res.dom, 4);
    updateBox(styleListBox, res.style, 7);
  };

  rust
    .then(_renderer => {
      renderer = _renderer;
      render();
    })
    .catch(console.error);

  htmlBox.addEventListener("keyup", render);
  cssBox.addEventListener("keyup", render);

  // hacky hack
  // force prism-live to resize text boxes
  window.dispatchEvent(new Event("resize"));
  render();

  const setTabVisible = type => {
    displayListBox.parentNode.style.display =
      type === "display" ? "block" : "none";
    styleListBox.parentNode.style.display = type === "style" ? "block" : "none";
    domListBox.parentNode.style.display = type === "dom" ? "block" : "none";
    canvasWrapper.style.display = type === "canvas" ? "block" : "none";
  };

  const addTabBtn = (id, type) => {
    const node = document.getElementById(id);
    node.addEventListener("click", () => {
      setTabVisible(type);
    });
  };

  addTabBtn("display-list-btn", "display");
  addTabBtn("style-list-btn", "style");
  addTabBtn("dom-list-btn", "dom");
  addTabBtn("canvas-btn", "canvas");
};
