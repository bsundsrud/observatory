import m from 'mithril';
import VizceralComponent from './vizceralComponent';
import style from './observatoryComponent.css';

class ObservatoryControls {
    view(vnode) {
        return m("div.observatory-controls", [
            m(ObservatoryBreadcrumb, {
                graphs: vnode.attrs.graphs,
                selectionChanged: vnode.attrs.graphSelectionChanged,
                graphSelectionLoader: vnode.attrs.graphSelectionLoader,
                viewChanged: vnode.attrs.viewChanged,
                graph: vnode.attrs.graph,
                path: vnode.attrs.path,
            }),
            m("a.button", {
                onclick: vnode.attrs.onRefresh || (() => {})
            }, "Refresh")
        ]);
    }
}

class GraphPicker {

    oninit(vnode) {
        this.onSelection = vnode.attrs.onSelection || (() => {});
        this.loadOptions(vnode);
    }

    loadOptions(vnode) {
        const loader = vnode.attrs.loader || (() => new Promise((resolve) => resolve([])));
        vnode.state.loading = true;
        loader().then((v) => {
            vnode.state.options = v;
            if (v.length > 0) {
                this.select(v[1]);
            }
            vnode.state.loading = false;
        });
    }

    select(id) {
        this.onSelection(id);
    }

    view(vnode) {
        const options = vnode.state.options || [];
        const changeHandler = vnode.attrs.onSelection || (() => {});
        const selected = vnode.attrs.selected || null;

        const createOption = (id, name, selected) => {
            var opts = { value: id };
            if (selected) {
                opts['selected'] = true;
            }
            return m("option", opts, name);
        };
        if (vnode.state.loading) {
            return m("select", m("option", "Loading..."));
        } else {
            return m("select",
                     { oninput: m.withAttr("value", this.select, this) },
                     options.map((o) => createOption(o, o, selected === o)));
        }
    }
}

function createPathSegment(paths, path, changeHandler) {
    let index = paths.findIndex((v) => v === path);
    let viewPath = paths.slice(0, index + 1);
    return m("span.path-segment", [
        m("span.separator", "/"),
        m("span.path-name",
          m("a.view-link", { onclick: () => changeHandler(viewPath) }, path))
    ]);
}

class ObservatoryBreadcrumb {
    view(vnode) {
        const graph = vnode.attrs.graph || '';
        const path = vnode.attrs.path || [];
        const selectionChanged = vnode.attrs.selectionChanged || (() => {});
        const viewNavigation = vnode.attrs.viewChanged || (() => {});
        
        return m("div.observatory-breadcrumb", [
            m("span.graph-title", "Graph:"),
            m(GraphPicker, {
                selected: graph,
                onSelection: selectionChanged,
                loader: vnode.attrs.graphSelectionLoader,
            }),
            m("span.graph-name", {onclick: () => viewNavigation([]) },"root"),
            path.map((p) => createPathSegment(path, p, viewNavigation))
        ]);
    }
}


export default class ObservatoryComponent {
    constructor(vnode) {
        this.state = vnode.attrs.state;
    }

    oninit(vnode) {
        this.state.fetchGraphList();
    }

    refreshClicked(ev) {
        this.state.refresh();
    }

    graphSelectionChanged(v) {
        this.state.fetchGraph(v);
    }

    breadcrumbNavigation(v) {
        this.state.view = v;
        console.log("View changed via breadcrumb", v);
    }

    viewChanged(g) {
        this.state.view = g.view;
        console.log("View changed", g);
        m.redraw();
    }

    view(vnode) {
        console.log("Rendering Observatory", this.state.view);
        return m("div.observatory", [
            m(ObservatoryControls, {
                graph: this.state.currentGraph,
                path: this.state.view,
                graphSelectionLoader: this.state.fetchGraphList.bind(this.state),
                graphSelectionChanged: this.graphSelectionChanged.bind(this),
                viewChanged: this.breadcrumbNavigation.bind(this),
                onRefresh: this.refreshClicked.bind(this),
                
            }),
            m(VizceralComponent, {
                traffic: this.state.traffic,
                allowDraggingOfNodes: true,
                view: this.state.view,
                viewChanged: this.viewChanged.bind(this)
            })
        ]);
    }
}
