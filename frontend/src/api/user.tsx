export default class User {
    static regexUsername = /^[\w]{4,20}$/;
    static regexPassword = /^[\w]{8,64}$/;
    static regexMail = /^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$/;
    static async login(name: string, password: string): Promise<number> {
        if (!User.regexUsername.test(name)) return 1;
        if (!User.regexPassword.test(password)) return 2;

        try {
            const response = await fetch('http://localhost:8080/api/user/login', {
                method: "POST",
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ username: name, password }),
                credentials: 'include'
            });

            if (!response.ok) return response.status; // Devuelve código HTTP si error
            return 0; // OK
        } catch (error) {
            return -1; // Error de red u otro
        }
    }
    static async register(name: string, mail: string, password: string): Promise<number> {
        if (!User.regexUsername.test(name)) return 1;
        if (!User.regexPassword.test(password)) return 2;
        if (!User.regexMail.test(mail)) return 3;

        try {
            const response  = await fetch('http://localhost:8080/api/user/register', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({username: name, mail: mail, password: password}),
                credentials: 'include'
            });

            if (!response.ok) return response.status;
            return 0;
        } catch (error) {
            return -1;
        }
    }
    static async logout() {
        try {
            const response  = await fetch('http://localhost:8080/api/user/logout', {
                method: 'POST',
                credentials: 'include'
            });

            if (!response.ok) return response.status;
            return 0;
        } catch (error) {
            return -1;
        }
    }
}
