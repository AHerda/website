function rainbow() {
    console.log("raibow function called");
    let paper = document.querySelector("canvas.visual");
    if (paper == null) {
        console.log("Canvas not found");
    } else {
        console.log("Canvas found");
    }
    paper.width = window.innerWidth * 0.7;
    paper.height = paper.width * 0.5;

    const pen = paper.getContext("2d");
    pen.clearRect(0, 0, paper.width, paper.height);

    const settings = {
        start: Date.now(),
        duration: 600,              // In seconds
        rainbowColors: ["#ff0000", "#ff8700", "#ffd300", "#deff0a", "#a1ff0a", "#0aff99", "#0aefff", "#147df5", "#580aff", "#be0aff"],
        mainColor: "#ffffff",
        numberOfArcs: 10,
        numberOfTurns: 100,
        opacityChangeDuration: 3,   // In seconds
        sound: false
    };

    function speed(i) {
        return (settings["numberOfTurns"] - i) * 2 * Math.PI / (settings["duration"]);
    }

    function position(centerWidth, centerHeight, radius, angle) {
        return {x: centerWidth - (Math.cos(angle) * radius),
            y: centerHeight - (Math.sin(angle) * radius)
        }
    }

    function drawArc(width, height, radius, color) {
        pen.beginPath();
        pen.strokeStyle = color;
        pen.lineWidth = 3;
        pen.arc(width, height, radius, Math.PI, 2 * Math.PI);
        pen.stroke();
    }

    function drawPoint(centerWidth, centerHeight, radius, color, time, i) {
        // Obliczanie parametrów
        let angularSpeed = speed(i);
        let angle = Math.abs(angularSpeed * time % (2 * Math.PI));
        angle = angle < Math.PI ? angle : 2 * Math.PI - angle;

        // Rysowanie kropki
        pen.beginPath();
        let pos = position(centerWidth, centerHeight, radius, angle);
        pen.arc(pos.x, pos.y, paper.height * .02, 0, 2 * Math.PI);
        pen.fillStyle = color;
        pen.shadowColor = color;
        pen.shadowBlur = 10;
        pen.fill();
        pen.shadowBlur = 0;

    }

    function calculateTimeToCollision(i, timeOfPreviousContact) {
        return timeOfPreviousContact + (1000 * settings["duration"] / (settings["numberOfTurns"] - i)) / 2;
    }

    let info;

    function init() {
        info = settings["rainbowColors"].map((color, index) => {
            const timeOfPreviousCollision = 0;
            const timeOfNextCollision = calculateTimeToCollision(index, timeOfPreviousCollision);

            const audio = new Audio(`/static/sounds/vibraphone-key-${index}.wav`);
            audio.playbackRate = 1.6;
            audio.volume = 0.5;

            return {color, timeOfPreviousCollision, timeOfNextCollision, audio};
        });
    }

    function run() {
        pen.fillStyle = "rgb(18, 18, 18)";
        pen.fillRect(0, 0, paper.width, paper.height);

        let runTime = Date.now() - settings["start"];

        let width = paper.width;
        let height = paper.height;

        pen.beginPath();
        pen.strokeStyle = settings["mainColor"];
        pen.lineWidth = 3;
        pen.moveTo(width * .1, height * .84);
        pen.lineTo(width * .9, height * .84);
        pen.stroke();

        info.forEach((arc, i) => {
            let radius = (i + 1) * (height * .68) / settings["numberOfArcs"];

            drawArc(width / 2, height * .82, radius, settings["mainColor"]);

            pen.globalAlpha = Math.max(1 - ((runTime - arc.timeOfPreviousCollision) / (settings["opacityChangeDuration"] * 1000)), 0);
            drawArc(width / 2, height * .82, radius, arc.color);
            pen.globalAlpha = 1;

            if (runTime > arc.timeOfNextCollision) {
                let temp = arc.timeOfPreviousCollision;
                arc.timeOfPreviousCollision = arc.timeOfNextCollision;
                arc.timeOfNextCollision = calculateTimeToCollision(i, temp);

                if (settings["sound"]) {
                    arc.audio.play();
                }
            }

            drawPoint(width / 2, height * .82, radius, arc.color, runTime / 1000, i);
        });

        requestAnimationFrame(run);
    }

    paper.onclick = () => {
        settings["sound"] = !settings["sound"];
    };

    init();
    run();
}
