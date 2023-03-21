# monocole

Different Types Of Modes:
    Light weight mode
        -> Only an agent is running on your computer and we connect to it 
            -> Has its own storage
        -> What are we going to use for storage, Persistant storage
        -> Connect to it 
    Controller Mode
        -> A centalized controller that connects to all agents and provides a centralized location
        -> Can have its own storage or light weight
            -> Different Types
                -> What is default(Local Persistant Storage)
                -> Cassandra, MongoDB, Local Persistant Storage
    Api Mode
        -> A light weight container that doesn't do anything but provides an interface to connect too

What do we collect and want to do??
    -> System Information
        -> What is the system?
        -> Agent will collect this on every boot
        -> Store in DB, or local Persistance Storage
    -> Real Time Analytics 
        -> Counters(Don't have to log) -> only specified
            -> will take up alot of space 
    -> Control
        -> ability to turn on and off computers 
        -> Update Computesr


PlantUML:

@startuml
class Storage {
 fn load(StorageClass)
 fn connect()
}
interface StorageClass {
 fn insert()
 fn bulk_insert()
 fn create_table()
 fn create_db()
 fn drop()
 fn del()
 fn connect()
}

struct mongo {
 fn insert()
 fn bulk_insert()
 fn create_table()
 fn create_db()
 fn drop()
 fn del()
 fn connect()
}
struct polodb {
+ file_path
 fn insert()
 fn bulk_insert()
 fn create_table()
 fn create_db()
 fn drop()
 fn del()
 fn connect()
}

struct cassandra {
 fn insert()
 fn bulk_insert()
 fn create_table()
 fn create_db()
 fn drop()
 fn del()
 fn connect()
}
StorageClass --> mongo
StorageClass --> polodb
StorageClass --> cassandra
Storage --> StorageClass
@enduml