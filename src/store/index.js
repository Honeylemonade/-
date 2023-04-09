import {defineStore} from 'pinia'

export const useConfigStore = defineStore('config', {
    state: () => {
        return {
            config: {
                common: {
                    backend_endpoint: "",
                    db_endpoint: "",
                    db_user: "",
                    db_password: "",
                    db_database: ""
                },
                modelConfs: []
            }
        }
    },
    getters: {},
    actions: {}
})