import('./src')
    .then(data => {
        const application = new data.Application();
        application.run();
    })
    .catch(console.error);
