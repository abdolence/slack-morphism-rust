// Nothing is fetched until the first Convert: the module is ~250 KB gzipped
// and most readers of this page never press the button.
(function () {
  "use strict";

  var MOUNT_ID = "blockkit-to-rust";
  var SCRIPT_SRC = document.currentScript ? document.currentScript.src : "";
  var PLACEHOLDER =
    '{\n  "blocks": [\n    { "type": "section", "text": { "type": "mrkdwn", "text": "hello" } }\n  ]\n}';

  var modulePromise = null;

  function assetsBase(mount) {
    var dir = mount.getAttribute("data-assets") || "blockkit/";
    if (typeof path_to_root === "string") {
      return path_to_root + dir;
    }
    // Outside mdbook's bootstrap, derive the base from this script's own URL.
    return SCRIPT_SRC.replace(/assets\/blockkit-widget\.js.*$/, "") + dir;
  }

  function loadConverter(base) {
    if (!modulePromise) {
      // path_to_root can be "", which makes base a bare specifier
      // ("blockkit/..."); dynamic import() only accepts relative or
      // absolute URLs, so resolve against the document before importing.
      var moduleUrl = new URL(base + "blockkit_to_rust.js", document.baseURI).href;
      var wasmUrl = new URL(base + "blockkit_to_rust_bg.wasm", document.baseURI).href;
      modulePromise = import(moduleUrl).then(function (mod) {
        return mod.default({ module_or_path: wasmUrl }).then(function () {
          return mod;
        });
      });
    }
    return modulePromise;
  }

  function element(tag, className, text) {
    var node = document.createElement(tag);
    if (className) {
      node.className = className;
    }
    if (text) {
      node.textContent = text;
    }
    return node;
  }

  function build(mount) {
    mount.textContent = "";
    mount.classList.add("blockkit-widget");

    var label = element("label", "blockkit-label", "Block Kit Builder JSON");
    label.setAttribute("for", "blockkit-input");

    var input = element("textarea", "blockkit-input");
    input.id = "blockkit-input";
    input.rows = 14;
    input.spellcheck = false;
    input.placeholder = PLACEHOLDER;

    var convert = element("button", "blockkit-convert", "Convert");
    convert.type = "button";

    var styleLabel = element("label", "blockkit-option", "Output ");
    var style = document.createElement("select");
    style.appendChild(new Option("Builders", "builders", true, true));
    style.appendChild(new Option("Raw serde_json::from_value", "raw"));
    styleLabel.appendChild(style);

    var emojiLabel = element("label", "blockkit-option");
    var emoji = document.createElement("input");
    emoji.type = "checkbox";
    emoji.checked = true;
    emojiLabel.appendChild(emoji);
    emojiLabel.appendChild(document.createTextNode(" Treat emoji: true as default"));

    var controls = element("div", "blockkit-controls");
    controls.appendChild(convert);
    controls.appendChild(styleLabel);
    controls.appendChild(emojiLabel);

    var wrap = element("div", "blockkit-output-wrap");
    var pre = element("pre", "blockkit-output");
    var code = element("code", "language-rust hljs");
    pre.appendChild(code);
    var copy = element("button", "blockkit-copy", "Copy");
    copy.type = "button";
    wrap.appendChild(pre);
    wrap.appendChild(copy);

    var error = element("p", "blockkit-error");
    error.setAttribute("role", "alert");
    error.setAttribute("aria-live", "polite");
    var warnings = element("ul", "blockkit-warnings");

    mount.appendChild(label);
    mount.appendChild(input);
    mount.appendChild(controls);
    mount.appendChild(wrap);
    mount.appendChild(error);
    mount.appendChild(warnings);

    return {
      mount: mount,
      input: input,
      convert: convert,
      style: style,
      emoji: emoji,
      code: code,
      copy: copy,
      error: error,
      warnings: warnings
    };
  }

  function render(ui, envelope) {
    ui.error.textContent = "";
    ui.warnings.textContent = "";

    if (!envelope.ok) {
      ui.code.textContent = "";
      // message already carries the path (see ConvertError's Display impl);
      // e.path is redundant here and re-adding it would print it twice.
      ui.error.textContent = envelope.errors
        .map(function (e) {
          return e.message;
        })
        .join("\n");
      return;
    }

    ui.code.textContent = envelope.code;
    ui.code.removeAttribute("data-highlighted");
    if (window.hljs) {
      if (hljs.highlightElement) {
        hljs.highlightElement(ui.code);
      } else if (hljs.highlightBlock) {
        hljs.highlightBlock(ui.code);
      }
    }

    if (envelope.errors.length) {
      ui.error.textContent =
        envelope.errors
          .map(function (e) {
            return e.message;
          })
          .join("\n") + " — open an issue if this block should be supported.";
    }
    envelope.warnings.forEach(function (w) {
      ui.warnings.appendChild(element("li", null, w.message));
    });
  }

  function run(ui, base) {
    var source = ui.input.value.trim();
    if (!source) {
      ui.error.textContent = "Paste some Block Kit JSON first.";
      return;
    }
    var original = ui.convert.textContent;
    ui.convert.disabled = true;
    ui.convert.textContent = "Loading converter";

    loadConverter(base)
      .then(function (mod) {
        ui.convert.textContent = original;
        var options = JSON.stringify({
          emoji_true_is_default: ui.emoji.checked,
          style: ui.style.value
        });
        render(ui, JSON.parse(mod.convert_json(source, options)));
      })
      .catch(function (e) {
        ui.code.textContent = "";
        ui.error.textContent =
          "The converter could not be loaded; it is built by the site's deploy " +
          "workflow. (" + e + ")";
      })
      .then(function () {
        ui.convert.disabled = false;
        ui.convert.textContent = original;
      });
  }

  function copyOutput(ui) {
    var text = ui.code.textContent;
    if (!text) {
      return;
    }
    var done = function () {
      ui.copy.textContent = "Copied";
      window.setTimeout(function () {
        ui.copy.textContent = "Copy";
      }, 1500);
    };
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text).then(done, function () {
        legacyCopy(text, done);
      });
    } else {
      legacyCopy(text, done);
    }
  }

  function legacyCopy(text, done) {
    var scratch = document.createElement("textarea");
    scratch.value = text;
    scratch.setAttribute("readonly", "");
    scratch.style.position = "absolute";
    scratch.style.left = "-9999px";
    document.body.appendChild(scratch);
    scratch.select();
    try {
      document.execCommand("copy");
      done();
    } finally {
      document.body.removeChild(scratch);
    }
  }

  function init() {
    var mount = document.getElementById(MOUNT_ID);
    if (!mount) {
      return;
    }
    var base = assetsBase(mount);
    var ui = build(mount);
    ui.convert.addEventListener("click", function () {
      run(ui, base);
    });
    ui.input.addEventListener("keydown", function (e) {
      if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
        e.preventDefault();
        run(ui, base);
      }
    });
    ui.copy.addEventListener("click", function () {
      copyOutput(ui);
    });
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
