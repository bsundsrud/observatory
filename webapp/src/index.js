import m from "mithril";
import traffic from './data/static';
import VizceralComponent from './vizceral/component';
import style from './style.css';

class MainComponent {
    view(vnode) {
        return m(VizceralComponent, { 
            traffic,
            allowDraggingOfNodes: true
         });
    }
}

m.mount(document.body, MainComponent);