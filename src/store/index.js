import { defineStore } from 'pinia'

export const useConfigStore = defineStore('config', {
    state: () => {
        return {
            config: {
                common: {
                    backend_endpoint: "10.229.181.166:8600",
                    db_endpoint: "10.229.181.166:8306",
                    db_user: "root",
                    db_password: "bce_cloud#",
                    db_database: "objectmanager"
                },
                models_conf: []
            }
        }
    },
    getters: {},
    actions: {}
})