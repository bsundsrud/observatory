import m from "mithril";
import traffic from './data/static';
import VizceralComponent from './vizceral/component';
import style from './style.css';
var Data = {
    state: {},
    fetch() {
        m.request({
            method: "GET",
            url: "http://localhost:8081/api/state/Global"
        }).then((resp) => Data.state = resp);
    }
};

class MainComponent {
    oninit() {
        Data.fetch()
    }
    view(vnode) {
        return m(VizceralComponent, { 
            traffic: Data.state,
            allowDraggingOfNodes: true
         });
    }
}

m.mount(document.body, MainComponent);