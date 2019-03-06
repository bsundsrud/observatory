import m from "mithril";
import traffic from './data/static';
import VizceralComponent from './components/vizceralComponent';
import normalize from 'normalize.css';
import style from './style.css';
import ObservatoryState from './state/observatory';
import ObservatoryComponent from './components/observatoryComponent';

var State = new ObservatoryState("http://localhost:8081");

class MainComponent {

    view(vnode) {
        return m(ObservatoryComponent, {
            state: State
        });
    }
}

m.mount(document.body, MainComponent);
