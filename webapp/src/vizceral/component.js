import Vizceral from 'vizceral';
import m from 'mithril';
import { isEqual, merge } from 'lodash';

import style from './style.css';

function getPerformanceNow() {
    const g = window;
    if (g != null) {
        const perf = g.performance;
        if (perf != null) {
            try {
                const perfNow = perf.now();
                if (typeof perfNow === 'number') {
                    return perfNow;
                }
            } catch (e) {
                // do nothing
            }
        }
    }
    return null;
}

class VizceralComponent {
    view(vnode) {
        return m("div.vizceral", [
            m(VizceralCanvasComponent, vnode.attrs),
            m("div.vizceral-notice")
        ])
    }
}

const defaults = {
    connectionHighlighted: () => { },
    definitions: {},
    filters: [],
    match: '',
    nodeHighlighted: () => { },
    nodeUpdated: () => { },
    nodeContextSizeChanged: () => { },
    matchesFound: () => { },
    objectHighlighted: () => { },
    objectHovered: () => { },
    objectToHighlight: null,
    showLabels: true,
    allowDraggingOfNodes: false,
    styles: {},
    traffic: {},
    viewChanged: () => { },
    viewUpdated: () => { },
    view: [],
    targetFramerate: null
};

class VizceralCanvasComponent {
    oncreate(vnode) {
        const props = merge(defaults, vnode.attrs);
        this.vizceral = new Vizceral(vnode.dom, props.targetFramerate);
        this.updateStyles(props.styles);
        this.vizceral.on('viewChanged', props.viewChanged);
        this.vizceral.on('objectHighlighted', props.objectHighlighted);
        this.vizceral.on('objectHovered', props.objectHovered);
        this.vizceral.on('nodeUpdated', props.nodeUpdated);
        this.vizceral.on('nodeContextSizeChanged', props.nodeContextSizeChanged);
        this.vizceral.on('matchesFound', props.matchesFound);
        this.vizceral.on('viewUpdated', props.viewUpdated);
        this.vizceral.setOptions({
            allowDraggingOfNodes: props.allowDraggingOfNodes,
            showLabels: props.showLabels
        });
        if (props.filters) {
            this.vizceral.setFilters(props.filters);
        }

        if (props.definitions) {
            this.vizceral.updateDefinitions(props.definitions);
        }

        setTimeout(() => {
            this.vizceral.setView(props.view, props.objectToHighlight);
            this.vizceral.updateData(props.traffic);
            this.vizceral.animate();
            this.vizceral.updateBoundingRectCache();
        }, 0);
    }

    onbeforeupdate(vnode, old) {
        const nextProps = vnode.attrs;
        const oldProps = old.attrs;
        if (!isEqual(nextProps.styles, oldProps.styles)) {
            this.updateStyles(nextProps.styles);
        }
        if (!isEqual(nextProps.view, oldProps.view)
            || !isEqual(nextProps.objectToHighlight, oldProps.objectToHighlight)) {
            this.vizceral.setView(nextProps.view, nextProps.objectToHighlight);
        }

        if (!isEqual(nextProps.filters, oldProps.filters)) {
            this.vizceral.setFilters(nextProps.filters);
        }
        if (!isEqual(nextProps.showLabels, oldProps.showLabels)
            || !isEqual(nextProps.allowDraggingOfNodes, oldProps.allowDraggingOfNodes)) {
            this.vizceral.setOptions({
                allowDraggingOfNodes: nextProps.allowDraggingOfNodes,
                showLabels: nextProps.showLabels
            });
        }
        if (!isEqual(nextProps.modes, oldProps.modes)) {
            this.vizceral.setModes(nextProps.modes);
        }

        if (!isEqual(nextProps.definitions, oldProps.definitions)) {
            this.vizceral.updateDefinitions(nextProps.definitions);
        }

        if (nextProps.match !== oldProps.match) {
            this.vizceral.findNodes(nextProps.match);
        }
        // If the data does not have an updated field, just assume it's modified now
        // This also solves the case between data updates
        nextProps.traffic.updated = nextProps.traffic.updated || Date.now();
        if (!oldProps.traffic.nodes
            || nextProps.traffic.updated > (oldProps.traffic.updated || 0)) {
            this.vizceral.updateData(nextProps.traffic);
        }
    }

    updateStyles(styles) {
        const styleNames = this.vizceral.getStyles();
        const customStyles = styleNames.reduce((result, styleName) => {
            result[styleName] = styles[styleName] || result[styleName];
            return result;
        }, {});
        this.vizceral.updateStyles(customStyles);
    }

    onremove(vnode) {
        delete this.vizceral;
    }

    view(vnode) {
        return m("canvas", {
            style: {
                width: "100%",
                height: "100%"
            }
        })
    }
}

export default VizceralComponent;