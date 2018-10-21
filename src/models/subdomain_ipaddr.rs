use errors::*;
use diesel::prelude::*;
use models::*;
use std::net;


#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Subdomain)]
#[belongs_to(IpAddr)]
#[table_name="subdomain_ipaddrs"]
pub struct SubdomainIpAddr {
    pub id: i32,
    pub subdomain_id: i32,
    pub ip_addr_id: i32,
}

impl Model for SubdomainIpAddr {
    type ID = (i32, i32);

    fn list(db: &Database) -> Result<Vec<Self>> {
        use schema::subdomain_ipaddrs::dsl::*;

        let results = subdomain_ipaddrs.load::<Self>(db.db())?;

        Ok(results)
    }

    fn filter(db: &Database, filter: &Filter) -> Result<Vec<Self>> {
        use schema::subdomain_ipaddrs::dsl::*;

        let query = subdomain_ipaddrs.filter(filter.sql());
        let results = query.load::<Self>(db.db())?;

        Ok(results)
    }

    fn by_id(db: &Database, my_id: i32) -> Result<Self> {
        use schema::subdomain_ipaddrs::dsl::*;

        let subdomain_ipaddr = subdomain_ipaddrs.filter(id.eq(my_id))
            .first::<Self>(db.db())?;

        Ok(subdomain_ipaddr)
    }

    fn id(db: &Database, query: &Self::ID) -> Result<i32> {
        use schema::subdomain_ipaddrs::dsl::*;

        let (my_subdomain_id, my_ip_addr_id) = query;
        let subdomain_ipaddr_id = subdomain_ipaddrs.filter(subdomain_id.eq(my_subdomain_id))
                                                   .filter(ip_addr_id.eq(my_ip_addr_id))
                                                   .select(id)
                                                   .first::<i32>(db.db())?;

        Ok(subdomain_ipaddr_id)
    }

    fn id_opt(db: &Database, query: &Self::ID) -> Result<Option<i32>> {
        use schema::subdomain_ipaddrs::dsl::*;

        let (my_subdomain_id, my_ip_addr_id) = query;
        let subdomain_ipaddr_id = subdomain_ipaddrs.filter(subdomain_id.eq(my_subdomain_id))
                                                   .filter(ip_addr_id.eq(my_ip_addr_id))
                                                   .select(id)
                                                   .first::<i32>(db.db())
                                                   .optional()?;

        Ok(subdomain_ipaddr_id)
    }
}

pub struct PrintableSubdomainIpAddr {
    subdomain: String,
    ipaddr: net::IpAddr,
}

impl fmt::Display for PrintableSubdomainIpAddr {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{:?} -> {}", self.subdomain, self.ipaddr)
    }
}

impl Printable<PrintableSubdomainIpAddr> for SubdomainIpAddr {
    fn printable(&self, db: &Database) -> Result<PrintableSubdomainIpAddr> {
        let subdomain = Subdomain::by_id(db, self.subdomain_id)?;
        let ipaddr = IpAddr::by_id(db, self.ip_addr_id)?;
        Ok(PrintableSubdomainIpAddr {
            subdomain: subdomain.value.to_string(),
            ipaddr: ipaddr.value.parse()?,
        })
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="subdomain_ipaddrs"]
pub struct NewSubdomainIpAddr {
    pub subdomain_id: i32,
    pub ip_addr_id: i32,
}

impl Printable<PrintableSubdomainIpAddr> for NewSubdomainIpAddr {
    fn printable(&self, db: &Database) -> Result<PrintableSubdomainIpAddr> {
        let subdomain = Subdomain::by_id(db, self.subdomain_id)?;
        let ipaddr = IpAddr::by_id(db, self.ip_addr_id)?;
        Ok(PrintableSubdomainIpAddr {
            subdomain: subdomain.value.to_string(),
            ipaddr: ipaddr.value.parse()?,
        })
    }
}
