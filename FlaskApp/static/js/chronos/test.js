var props;
var resolution;

const load_flask_props = () => {
  props = JSON.parse(navbar.dataset["props"]);
};

const setup_page_layout = () => {
  // function for setting up navbar
  const setup_navbar = () => {
    const navbar = document.getElementById("navbar");
    // navbar.style = "width: 100%; border: 1px solid white; padding: 10px";
    const mdb_hierarchy = props["mdb_hierarchy"];
    const daily_datasets = mdb_hierarchy["stats"]["time series"]["daily"];
    console.log(mdb_hierarchy);
    for (let k in daily_datasets) {
      let cat = document.createElement("div");
      cat.className = "dropdown";
      cat.style = "padding-right: 30px; height: 100%}";
      let title = document.createElement("p");
      title.innerHTML = k;
      title.style = "color: white";
      cat.append(title);
      let dropdown_content = document.createElement("div");
      dropdown_content.className = "dropdown-content";
      dropdown_content.style = "background-color: #222222";
      for (let k2 in daily_datasets[k]) {
        console.log(k2);
        let title = document.createElement("p");
        title.innerHTML = k2;
        title.style = "height: 100%; margin-bottom: 0px";
        dropdown_content.append(title);
        cat.append(dropdown_content);
      }
      // cat.innerHTML = k;
      navbar.append(cat);
    }
    content.append(navbar);
  };
  // function for setting up plotting
  const setup_section = () => {
    const section = document.createElement("div");
    section.style = "color: white; width: 100%";
    section.className = "section";
    content.append(section);
  };

  const content = document.getElementById("content");
  setup_navbar();
  setup_section();
};

const load_ts = () => {
  console.log("ayy");
};

const display_heatmap = (resolution) => {
  // load_ts();
};

const add_event_listeners = () => {};

const init = () => {
  resolution = "daily"; // TODO: make changeable

  load_flask_props();
  setup_page_layout();
  display_heatmap(resolution);
  add_event_listeners();
};

init();
