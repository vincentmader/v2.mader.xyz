var W, H;
var chart;

export function init(canvas, ctx) {
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  var x = [];
  var y = [];
  var k3 = "";

  chart = new Chart(ctx, {
    type: "bar",
    data: {
      labels: x, //["Red", "Blue", "Yellow", "Green", "Purple", "Orange"],
      datasets: [
        {
          label: k3,
          data: y,
          backgroundColor: "green",
          // backgroundColor: [
          //   "rgba(255, 99, 132, 0.2)",
          //   "rgba(54, 162, 235, 0.2)",
          //   "rgba(255, 206, 86, 0.2)",
          //   "rgba(75, 192, 192, 0.2)",
          //   "rgba(153, 102, 255, 0.2)",
          //   "rgba(255, 159, 64, 0.2)",
          // ],
          borderColor: "green",
          // borderColor: [
          //   "rgba(255, 99, 132, 1)",
          //   "rgba(54, 162, 235, 1)",
          //   "rgba(255, 206, 86, 1)",
          //   "rgba(75, 192, 192, 1)",
          // "rgba(153, 102, 255, 1)",
          // "rgba(255, 159, 64, 1)",
          // ],
          borderWidth: 1,
        },
      ],
    },
    options: {
      scales: {
        //x: {
        //  ticks: {
        //    // autoSkip: false,
        //    callback: function (value, index, values) {
        //      console.log("aaa");
        //      return value;
        //      // const date = new Date(value);
        //      // if (date.getMonth() == 0) {
        //      // return date.getFullYear();
        //      // } else {
        //      //   return null;
        //      // }
        //    },
        //  },
        //},
      },

      // scales: {
      //   xAxes: [
      //     {
      //       display: true,
      //       ticks: {
      //         suggestedMin: 0,
      //         suggestedMax: 1,
      //       },
      //     },
      //   ],
      //   // y: {
      //   //   beginAtZero: true,
      //   // },
      // },
    },
  });
}

export function update(k, k2, k3, x, y) {
  console.log("displaying dataset: ", y);
  chart.data = {
    labels: x, //["Red", "Blue", "Yellow", "Green", "Purple", "Orange"],
    datasets: [
      {
        label: k3,
        data: y,
        backgroundColor: "green",
        borderColor: "green",
        borderWidth: 1,
        line: {
          backgroundColor: "blue",
          tension: 0,
        },
      },
    ],
  };
  chart.options = {
    scales: {
      xAxes: [
        {
          type: "time",
          time: {
            unit: "month",
            // displayFormats: {
            //   year: "YYYY",
            // },
          },
        },
      ],
      yAxes: [
        {
          display: true,
          ticks: {
            //min: 0,
            //max: 100,
            //beginAtZero: true,
            suggestedMin: 0,
          },

          //maximum: 1,
        },
      ],
      // scales: {
      //   x: {
      //     ticks: {
      //       autoSkip: false,
      //       callback: function (value, index, values) {
      //         // console.log("aaa");
      //         const date = new Date(value);
      //         // if (date.getMonth() == 0) {
      //         return date.getFullYear();
      //         // } else {
      //         //   return null;
      //         // }
      //       },
      //     },
      //   },
      //   // y: {
      //   beginAtZero: true,
      // },
      // },
    },
  };
  chart.update();
}

// export function main(canvas, ctx, k, k2, k3, x, y) {
//   W = canvas.getBoundingClientRect().width;
//   H = W;
//   canvas.width = W;
//   canvas.height = W;

//   var myChart = new Chart(ctx, {
//     type: "line",
//     data: {
//       labels: x, //["Red", "Blue", "Yellow", "Green", "Purple", "Orange"],
//       datasets: [
//         {
//           label: k3,
//           data: y,
//           backgroundColor: "green",
//           // backgroundColor: [
//           //   "rgba(255, 99, 132, 0.2)",
//           //   "rgba(54, 162, 235, 0.2)",
//           //   "rgba(255, 206, 86, 0.2)",
//           //   "rgba(75, 192, 192, 0.2)",
//           //   "rgba(153, 102, 255, 0.2)",
//           //   "rgba(255, 159, 64, 0.2)",
//           // ],
//           borderColor: "green",
//           // borderColor: [
//           //   "rgba(255, 99, 132, 1)",
//           //   "rgba(54, 162, 235, 1)",
//           //   "rgba(255, 206, 86, 1)",
//           //   "rgba(75, 192, 192, 1)",
//           // "rgba(153, 102, 255, 1)",
//           // "rgba(255, 159, 64, 1)",
//           // ],
//           borderWidth: 1,
//         },
//       ],
//     },
//     options: {
//       scales: {
//         xAxes: [
//           {
//             display: true,
//             ticks: {
//               suggestedMin: 0,
//               suggestedMax: 1,
//             },
//           },
//         ],
//         // y: {
//         //   beginAtZero: true,
//         // },
//       },
//     },
//   });
// }
