import m from 'mithril';

class ObservatoryState {

    constructor(baseUrl) {
        this.baseUrl = baseUrl;
        this.graphs = [];
        this.traffic = {};
        this.view = [];
        this.currentGraph = '';
    }
    
    fetchGraphList() {
        return m.request({
            method: "GET",
            url: `${this.baseUrl}/api/state`
        }).then((resp) => {
            this.graphs = resp.graphs;
            return this.graphs;
        });
    }
    
    fetchGraph(name) {
        console.log("Fetching graph", name);
        return m.request({
            method: "GET",
            url: `${this.baseUrl}/api/state/${name}`
        }).then((resp) => {
            if (name !== this.currentGraph) {
                console.log("Graph changed; new graph", name);
                this.currentGraph = name;
                this.view = [];
            }
            this.traffic = resp;
            return this.traffic;
        });
    }

    refresh() {
        return this.fetchGraph(this.currentGraph); 
    }

}

export default ObservatoryState;
