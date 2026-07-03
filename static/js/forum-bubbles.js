(function () {
  const hero = document.getElementById("hero-section");
  const container = document.getElementById("forum-bubbles");
  if (!container || !hero) return;

  const W = 180,
    H = 70,
    PAD = 12,
    MAX = 5,
    GAP = 8;
  let posts = [],
    active = [];

  function rand(a, b) {
    return a + Math.random() * (b - a);
  }

  function getZones() {
    const hRect = hero.getBoundingClientRect();
    const tEl = hero.querySelector("[data-bubble-block]");
    const tRect = tEl.getBoundingClientRect();
    const textLeft = tRect.left - hRect.left;
    const textRight = tRect.right - hRect.left;
    const textTop = tRect.top - hRect.top;
    const textBottom = tRect.bottom - hRect.top;
    const cw = hRect.width;
    const ch = hRect.height;
    const zones = [];

    if (textLeft - W - PAD > PAD)
      zones.push({ x: [PAD, textLeft - W - PAD], y: [PAD, ch - H - PAD] });
    if (cw - textRight - W - PAD > PAD)
      zones.push({
        x: [textRight + PAD, cw - W - PAD],
        y: [PAD, ch - H - PAD],
      });
    if (textTop - H - PAD > PAD)
      zones.push({ x: [PAD, cw - W - PAD], y: [PAD, textTop - H - PAD] });
    if (ch - textBottom - H - PAD > PAD)
      zones.push({
        x: [PAD, cw - W - PAD],
        y: [textBottom + PAD, ch - H - PAD],
      });

    return zones;
  }

  function overlaps(x, y) {
    for (const e of active) {
      if (
        Math.abs(x - parseFloat(e.dataset.x)) < W + GAP &&
        Math.abs(y - parseFloat(e.dataset.y)) < H + GAP
      )
        return true;
    }
    return false;
  }

  function findPos() {
    const zones = getZones();
    if (!zones.length) return null;
    for (let i = 0; i < 50; i++) {
      const z = zones[Math.floor(Math.random() * zones.length)];
      const x = rand(z.x[0], Math.max(z.x[0], z.x[1]));
      const y = rand(z.y[0], Math.max(z.y[0], z.y[1]));
      if (!overlaps(x, y)) return { x, y };
    }
    return null;
  }

  function removeEl(el) {
    el.remove();
    active = active.filter((e) => e !== el);
  }

  function startTimer(el) {
    clearTimeout(el._t);
    el._t = setTimeout(
      () => {
        if (el._hover) return;
        el.style.transition = "opacity 1.2s ease";
        el.style.opacity = "0";
        setTimeout(() => removeEl(el), 1200);
      },
      rand(4500, 8000),
    );
  }

  function spawn() {
    if (!posts.length || active.length >= MAX) return;
    const pos = findPos();
    if (!pos) return;

    const p = posts[Math.floor(Math.random() * posts.length)];
    const el = document.createElement("div");
    el.dataset.x = pos.x;
    el.dataset.y = pos.y;
    Object.assign(el.style, {
      position: "absolute",
      left: pos.x + "px",
      top: pos.y + "px",
      width: W + "px",
      opacity: "0",
      transition: "opacity 1s ease",
      zIndex: "2",
      pointerEvents: "auto",
      background: "rgba(255,255,255,0.95)",
      border: "1px solid #c7d2fe",
      borderRadius: "12px",
      padding: "8px 12px",
      boxShadow: "0 2px 8px rgba(52,72,211,0.10)",
      cursor: "default",
    });
    el.innerHTML = `<div style="font-size:11px;font-weight:700;color:#3448d3;margin-bottom:3px;">${p.alias}</div><div style="font-size:11px;color:#475569;line-height:1.45;overflow:hidden;display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;">${p.content}</div>`;

    el.onmouseenter = () => {
      el._hover = true;
      clearTimeout(el._t);
      Object.assign(el.style, {
        transition: "opacity 0.15s ease, box-shadow 0.15s ease",
        opacity: "1",
        zIndex: "20",
        boxShadow: "0 4px 20px rgba(52,72,211,0.20)",
      });
    };
    el.onmouseleave = () => {
      el._hover = false;
      Object.assign(el.style, {
        zIndex: "2",
        boxShadow: "0 2px 8px rgba(52,72,211,0.10)",
        transition: "opacity 1s ease, box-shadow 0.15s ease",
        opacity: "0.9",
      });
      startTimer(el);
    };

    container.appendChild(el);
    active.push(el);
    requestAnimationFrame(() =>
      requestAnimationFrame(() => {
        el.style.opacity = "0.9";
      }),
    );
    startTimer(el);
  }

  function loop() {
    spawn();
    setTimeout(loop, rand(1000, 2500));
  }

  fetch("/foro/preview")
    .then((r) => (r.ok ? r.json() : []))
    .then((data) => {
      if (data.length) {
        posts = data;
        requestAnimationFrame(() => requestAnimationFrame(loop));
      }
    })
    .catch(() => {});
})();
