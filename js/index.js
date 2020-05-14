const rust = import("../pkg/index.js");

const defaultHtml = `<div class="a">My Text</div>`;

//language=css
const defaultCss = `
 * { 
    display: block; 
    padding: 12px; 
 }
 
.a { 
    background: #ff0000; 
    color: #FFFFFF; 
}`;

let renderer;

window.onload = () => {
  if (NETLIFY) {
    document.getElementById("badge").style.display = "block";
  }

  const htmlBox = document.getElementById("htmlcode");
  const cssBox = document.getElementById("csscode");
  htmlBox.value = localStorage.getItem("html") || defaultHtml.trim();
  cssBox.value = localStorage.getItem("css") || defaultCss.trim();

  const displayBoxes = {};
  const setTabVisible = activeType => {
    Object.entries(displayBoxes).forEach(([type, node]) => {
      const parentNode =
        type === "canvas" || type === "iframe" ? node : node.parentNode;
      parentNode.style.display = type === activeType ? "block" : "none";
    });
  };

  const buttons = document.getElementById("header-buttons");
  const renderContent = document.getElementById("render-content");

  const addTabBtn = (text, type) => {
    const button = document.createElement("button");
    button.innerText = text;
    buttons.appendChild(button);

    let box = null;
    if (type === "canvas") {
      box = document.createElement("canvas");
      box.id = "canvas";
      renderContent.appendChild(box);
    } else if (type === "iframe") {
      box = document.createElement("iframe");
      box.id = "iframe";
      box.classList.add("iframe-box");
      renderContent.appendChild(box);
    } else {
      const wrapper = document.createElement("div");
      wrapper.classList.add("list-box");
      box = document.createElement("code");
      box.classList.add("language-json");
      wrapper.appendChild(box);
      renderContent.appendChild(wrapper);
    }

    displayBoxes[type] = box;
    button.addEventListener("click", () => {
      setTabVisible(type);
    });
  };

  addTabBtn("Renderer", "canvas");
  addTabBtn("Iframe ", "iframe");
  addTabBtn("Dom Nodes", "dom");
  addTabBtn("Style Nodes", "style");
  addTabBtn("Layout Nodes", "layout");
  addTabBtn("Display List", "display");

  const updateIframe = () => {
    const iframe = displayBoxes.iframe;
    if (!iframe) return;
    console.log(cssBox.value, htmlBox.value);
    const html = `<head>
        <style>
          head { display: none }
          body, html { padding: 0; margin: 0 }
          ${cssBox.value}
          </style>
     </head>
     <body>${htmlBox.value}</body>`;

    const blob = new Blob([html], { type: "text/html" });
    iframe.src = window.URL.createObjectURL(blob);
  };

  const updateBox = (type, value, level) => {
    const node = displayBoxes[type];
    node.innerHTML = "";
    node.appendChild(renderjson.set_show_to_level(level)(value));
  };

  const render = () => {
    if (!renderer) return;
    const res = renderer.render(htmlBox.value, cssBox.value);

    localStorage.setItem("css", cssBox.value);
    localStorage.setItem("html", htmlBox.value);

    // console.log( Prism.languages)
    updateBox("display", res.display_list, 3);
    updateBox("dom", res.dom, 4);
    updateBox("layout", res.layout, 3);
    updateBox("style", res.style, 7);
    updateIframe();
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
};
