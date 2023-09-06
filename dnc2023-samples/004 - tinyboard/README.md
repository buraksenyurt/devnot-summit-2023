# Tiny-Board

Bu örnekte amacım Rust, Actix, SeaORM ve Electron kullanılarak basit bir todo uygulaması geliştirmeye çalışmak. Örneği takip ettiğim [Udemy](https://www.udemy.com/course/build-a-todolist-with-actix-web-rust-and-electron-vue) kursundan esinlenerek geliştirmeye çalışıyorum.

## Database

Örnekte PostgreSQL kullanılmakta. Ben her zamanki gibi docker imajını tercih edeceğim.

```bash
sudo docker run -p 5434:5432 --name tinyboard -e POSTGRES_PASSWORD=tiger -d postgres
```

Buna göre uygulamanın veritabanı bağlantısını da tutan .env dosyasında aşağıdaki tanımı kullanmamız yeterli olacak.

```.env
DATABASE_URL=postgres://localhost:5434/tiny-board?user=postgres&password=tiger
```

## Rust Projesinin Oluşturulması

İlk olarak rust projesini oluşturuyoruz.

```bash
cargo new tiny-board-api
cd tiny-board-api

# Gerekli küfeler(crates) yüklenir
# Servis arayüzü ve CORS izinleri için Actix, 
# Ortam parametrelerini kullanabilmek için dotnev
# loglama için env_logger ve log

cargo add actix-web actix-cors dotenv serde_json env_logger log futures

# ORM aracı olarak da SeaORM
# ki örnekte postgresql kullanılıyor, asenkron çalışma zamanı içinse actix-runtime
cargo add sea-orm -F sqlx-postgres,runtime-actix-native-tls,macros

# ???
cargo add async-std -F attributes,tokio1

# json serileştirme desteği için standart serde
cargo add serde -F derive

# ???
cargo add tracing-subscriber -F env-filter
```

_**Not:** cargo.toml dosyasına eklenen bazı crate'ler için feature bildirilmleri söz konusu. Bu, ilgili crate'in hangi özelliklerinin aktifleştirileceği anlamına gelir. Yani sadece ihtiyacımız olan özellikleri alıp gereksiz bağımlılıklarının indirilmemesini sağlar._

## Migration İşlemleri

Pekçok ORM aracında olduğu gibi (diesel mesela) SeaORM için de migration işlerini kolaylaştıran bir tool var. Önce bunu sisteme yüklüyoruz.

```bash
cargo install sea-orm-cli

# istemci aracı kurduktan sonra aşağıdaki komut çalıştırılır
sea-orm-cli migrate init
```

Bu işlem migration isimli bir klasörün oluşturulmasını sağlar. İçerisinde yine bir rust kütüphanesi yer alır. Tablo tanımlamalarını, göç planlarını burada ayarlarız. Bu tabii geliştirme ortamı için de bir bağımlılıktır bu nedenle cargo.toml'a aşağıdaki dependencies bildiriminin de eklenmesi gerekir.

```toml
migration = { path = "migration" }

[dependencies.sea-orm-migration]
version = "^0"
features = [
    "runtime-actix-native-tls",
    "sqlx-postgres"
]
```

Bundan sonra ilk migration planını hazırlayabiliriz. Örneğin workitem tablosunun oluşturulması söz konusu olabilir.

```bash
sea-orm-cli migrate generate create_workitem_table
```

Oluşan dosyada tablo için gerekli alan bildirimleri yapıldıktan ve .env içeriği ayarlandıktan sonra migration klasörü altındayken cargo run ile planın yürütülmesi sağlanabilir.

```bash
cd migration
cargo run
```

Ben docker üzerinden başlattığım Postgresql örneğinde tiny-board isimli bir veritabanı oluşturmuştum. Dolayısıyla migration plan çalıştıktan sonra WorkItem tablosu burada üretildi. Ayrıca migration planlarına ait seaql_migrations isimli bir tablo da eklendi.

![assets/db_state_01.png](assets/db_state_01.png)

## Entity Oluşturma

Yukarıdaki işlemler bir migration planını işletip postgresql tarafında work_item isimli tablonun oluşturulmasını sağlamıştır. Yine sea-orm-cli aracını kullanarak bu tabloya karşılık gelecek entity veri yapısını da kolayca üretebiliriz. Proje ana klasörüne çıktıktan sonra aşağıdaki komutla ilerlenebilir.

```bash
cd ..
sea-orm-cli generate entity -o src/entity
```

Bu komut src/entity klasörü içerisinde work_item isimli veri yapısının oluşturulmasını sağlayacaktır. Elebette birden fazla tablo olması halinde her biri için birer entity modeli üretilir.

## Servis Tarafı için Testler

Web API tarafı tamamlandıktan sonra CRUD operasyonlarını test edebiliriz. Bunun için Postman veya curl gibi araçlar kullanılabilir. Tabii işlemlere başlamadan önce docker container olarak kullandığımız postgresql örneğinin çalıştığından emin olmalıyız.

**Yeni bir WorkItem eklemek için**

```text
Adres : 127.0.0.1:7000/workitems
Metot : HTTP Post
Örnek İçerik : 
{
    "title": "Hafta sonu 10 Km koşu",
    "summary": "Berlin maratonu öncesi hazırlıklar kapsamında yapılması planlanan koşudur.",
    "business_value": 100,
    "completed": false
}
```

![assets/api_request_01.png](assets/api_request_01.png)

**Tüm WorkItem'ları listelemek için**

```text
Adres : 127.0.0.1:7000/workitems
Metot : HTTP Get
```

![assets/api_request_02.png](assets/api_request_02.png)

**Belli bir ID bilgisine sahip WorkItem'ı getirmek için**

```text
Adres : 127.0.0.1:7000/workitems/4
Metot : HTTP Get
```

![assets/api_request_03.png](assets/api_request_03.png)

**Bir ID bilgisine sahip WorkItem'ı güncellemek için**

```text
Adres : 127.0.0.1:7000/workitems/4
Metot : HTTP Put
Örnek İçerik : 
{
    "title": "Rust X 5",
    "summary": "5 Pomodore'luk Rust çalışması",
    "business_value": 250,
    "completed": true
}
```

![assets/api_request_04.png](assets/api_request_04.png)

**Bir WorktItem'ı silmek için**

```text
Adres : 127.0.0.1:7000/workitems/3
Metot : HTTP Delete
```

![assets/api_request_05.png](assets/api_request_05.png)

Bu arada tüm bu işlemler sırasında debug moda ait tüm logları da terminal penceresinden görebiliriz.

![assets/api_request_runtime.png](assets/api_request_runtime.png)

## Electron Projesinin Oluşturulması

Önyüz uygulamamız Vue tabanlı bir Electron projesi. Eğer sistemde npm yüklü ise projenin kendisini scaffold şablonunu kullanarak kolayca oluşturabiliriz.

```bash
npm create @quick-start/electron
```

CLI tarafında sorulan soruları aşağıdaki gibi cevaplandırarak ilerleyebiliriz.

![assets/electron_setup.png](assets/electron_setup.png)

Önyüz uygulamasının ihtiyaç duyacağı npm paketlerini de yüklemek gerekiyor. Bunun için proje klasöründe aşağıdaki komutu vermek yeterli olacaktır.

```bash
cd tiny-board-app
npm i
```

Eğitimde [element-plus](https://element-plus.org/en-US/guide/installation.html) isimli bir bileşen seti kullanılmakta. Ayrıca rust ile yazılmış servis ile ilişim için axios paketi kullanılmakta. Bu paketleri yüklemek için aşağıdaki komutlarla ilerlenebilir.

```bash
npm install element-plus --save
npm install axios vue-axios --save
```

## Çalışma Zamanı

Örnek uygulama Postgresql veritabanını kullanmakta ve o da docker container olarak servis edilmekte. Dolayısıyla sistemde ilgili container'ın çalışır olduğundan emin olmak lazım. Bunun haricinde backend taraf servisinin de çalışır olması gerekiyor. Sonrasında eğer geliştirme modunda ilerleyeceksek **npm run dev** komutunu kullanabiliriz. Bu komut ile Electron uygulaması çalıştırılacaktır. Ayrıca localhost:5137 adresinden bu electron uygulamasının tarayıcıda çalışan versiyonuna da geçiş yapabiliriz. Bu özellikle uygulama ile servis tarafı arasındaki haberleşmeleri monitör etmek için idealdir. Çalıştığım örnekte CORS ihlallerine takıldığımdan rust uygulamasında buna yönelik bir değişiklik yapmak durumunda kaldım. İşte çalışma zamanına ait birkaç ekran görüntüsü.

Uygulama ilk açıldığında;

![assets/runtime_01.png](assets/runtime_01.png)

Örnek bir değişiklik yapıldıktan sonra;

![assets/runtime_02.png](assets/runtime_02.png)

Yeni bir Work Item eklendiğinde;

![assets/runtime_03.png](assets/runtime_03.png)

Bir Work Item silindiğinde;

![assets/runtime_04.png](assets/runtime_04.png)

![assets/runtime_05.png](assets/runtime_05.png)