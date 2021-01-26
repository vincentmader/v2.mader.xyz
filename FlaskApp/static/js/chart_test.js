canvas = document.getElementById("heatmapPlot");
var ctx = canvas.getContext("2d");
canvas.width = 400;
canvas.height = 400;

var myChart = new Chart(ctx, {
  type: "line",
  data: {
    labels: ["0", "pi", "2pi", "3pi", "4pi", "5pi"],
    datasets: [
      {
        label: "ayy",
        data: [1, -1, 1, -1, 1, -1],
        borderColor: "white",
        // [
        // "rgba(255, 99, 132, 1)",
        // "rgba(54, 162, 235, 1)",
        // "rgba(255, 206, 86, 1)",
        // "rgba(75, 192, 192, 1)",
        // "rgba(153, 102, 255, 1)",
        // "rgba(255, 159, 64, 1)",
        // ],
        borderWidth: 2,
      },
    ],
  },
  options: {
    scales: {
      yAxes: [
        {
          ticks: {
            beginAtZero: true,
          },
        },
      ],
    },
  },
});

// function drawRectangle(x1, y1, x2, y2) {
//   var width = Math.abs(x1 - x2);
//   var height = Math.abs(y1 - y2);
//   ctx.beginPath();
//   ctx.rect(x1, y1, width, height);
//   ctx.fillStyle = "green";
//   ctx.fill();
//   ctx.stroke();
// }

// function drawHeatmapCell(x, y) {
//   let cellWidth = 40;
//   let cellHeight = 40;

//   ctx.beginPath();
//   ctx.rect(x, y, cellWidth, cellHeight);
//   ctx.fillStyle = "green";
//   ctx.fill();
//   ctx.stroke();
// }

// const NUMBER_OF_ROWS = 5;
// const NUMBER_OF_COLS = 20;

// for (let j = 0; j < NUMBER_OF_ROWS; j++) {
//   for (let i = 0; i < NUMBER_OF_COLS; i++) {
//     drawHeatmapCell(50 * i, 50 * j);
//   }
// }
