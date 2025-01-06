const alphanum = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
let interval = null;

document.addEventListener("DOMContentLoaded", () => {
    let element = document.querySelector("h1#hacked");

    if (element != null) {
        element.onmouseover = event => {
            let iteration = 0;
            const tempo = 1 / 3;
            clearInterval(interval);

            interval = setInterval(() => {
                event.target.innerText = event.target.innerText
                    .split('')
                    .map((_, index) => {
                        if (index < iteration) {
                            return event.target.dataset.value[index];
                        }
                        return alphanum[Math.floor(Math.random() * alphanum.length)];
                    })
                    .join('');

                if (iteration >= event.target.dataset.value.length) {
                    clearInterval(interval);
                }
                iteration += tempo;
            }, 30);
        };
    }
    else {
        console.log("element is null");
    }
});
