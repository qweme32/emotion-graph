class Api {
  public token;
  constructor() {
    this.token = useCookie("token", { maxAge: 60 * 60 * 24 * 7 });
  }

  async check(): Promise<String | null> {
    if (!this.token.value || this.token.value == "") {
      return null;
    }

    const response = await $fetch('/api/user/me', {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${this.token.value}`
      }
    }).catch(e => console.dir(e)) as any;

    return response?.username;
  }

  async report(rate: number): Promise<boolean> {
    const response = await $fetch('/api/user/report', {
      method: 'POST',
      body: {
        rate
      },
      headers: {
        'Authorization': `Bearer ${this.token.value}`
      }
    }).catch(e => console.dir(e)) as any;

    return response ? true : false;
  }

  async reports(username: string, period: number): Promise<Array<any>> {
    const response = await $fetch('/api/user/reports', {
      method: 'POST',
      body: {
        username,
        period
      },
      headers: {
        'Authorization': `Bearer ${this.token.value}`
      }
    }).catch(e => console.dir(e)) as any;

    if (!response) {
      return [];
    }

    return response.reports.map((item: any) => {
      let date = new Date(parseInt(item.date.$date.$numberLong));

      date.setTime(date.getTime() - date.getTimezoneOffset() * 60 * 1000);

      return {
        date: date.toISOString(),
        rate: item.rate,
      }
    })
  }

  async authorize(username: string, password: string): Promise<boolean> {
    const response = await $fetch('/api/user/authorize', {
      method: 'POST',
      body: {
        username,
        password
      }
    }).catch(e => console.dir(e)) as any;

    if (!response) {
      return false;
    }

    this.token.value = response.token;

    return true;
  }
}

export const useApi = () => {
  return new Api();
}