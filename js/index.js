import("../pkg/index.js")
    .catch(console.error)
    .then((w) => {
        console.log(w.fast_add(41, 1));
        console.log(w.fast_sort([4, 5, 7, 9, 20], "sorting"));

        document.getElementById("num-btn").onclick = w.increment_counter;

        const canv = document.getElementById("packing-canvas");
        const ctx = canv.getContext("2d");
        canv.onclick = () => {
            const width = ctx.canvas.width;
            const height = ctx.canvas.height;
            const circles = w.pack_circles(width, height, 0.8, 5.0, 5.0);

            ctx.clearRect(0, 0, width, height);

            ctx.fillStyle = "#ebb43b";
            for (const c of circles) {
                ctx.beginPath();
                ctx.arc(c.x, c.y, c.r, 0.0, 2.0 * Math.PI);
                ctx.fill();
            }
        };
    });
