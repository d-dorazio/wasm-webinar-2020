// index.js
import("../pkg/index.js")
    .catch(console.error)
    .then((w) => {
        console.log(w.fast_add(41, 1));
        console.log(w.fast_sort([4, 5, 7, 9, 20], "sorting"));

        document.getElementById("num-btn").onclick = w.increment_counter;
    });
