use crate::common::*;
/// Indicates the various states of Central.
pub enum CentralState {
    Absent,
    PoweredOff,
    PoweredOn,
}
/// Indicates the various types of GATT event.
pub enum GattOperationType {
    Connection,
    Discovery,
}
/// Indicates the various types of characteristic write.
pub enum CharacteristicWriteType {
    WriteDefaultDeprecated,
    WriteWithResponse,
    WriteWithoutResponse,
}
/// Indicates the various types of characteristic operation.
pub enum CharacteristicOperationType {
    Read,
    Write,
    SubscribeToNotifications,
    UnsubscribeFromNotifications,
}
/// Indicates the various types of descriptor operation.
pub enum DescriptorOperationType {
    Read,
    Write,
}
/// Stores the manufacturer data
pub struct ManufacturerData {
    pub key: i64,
    pub data: String,
}
/// Stores the byte data of the advertisement packet sent by a Bluetooth device.
pub struct ScanRecord {
    pub name: String,
    pub uuids: Vec<String>,
    pub appearance: i64,
    pub tx_power: i64,
    pub manufacturer_data: Vec<ManufacturerData>,
}
/// Stores the advertisement packet information that is sent by a Bluetooth device.
pub struct ScanEntry {
    pub device_address: String,
    pub rssi: i64,
    pub scan_record: Box<ScanRecord>,
}
/** Describes the properties of a characteristic. This follows Bluetooth Core
Specification BT 4.2 Vol 3 Part G 3.3.1. Characteristic Properties.*/
pub struct CharacteristicProperties {
    pub broadcast: bool,
    pub read: bool,
    pub write_without_response: bool,
    pub write: bool,
    pub notify: bool,
    pub indicate: bool,
    pub authenticated_signed_writes: bool,
    pub extended_properties: bool,
}
/// Enable the BluetoothEmulation domain.
pub struct BluetoothEmulationEnableParams {
    pub state: Box<CentralState>,
    pub le_supported: bool,
}
/// Enable the BluetoothEmulation domain.
pub type BluetoothEmulationEnableReturns = ();
/// Set the state of the simulated central.
pub struct BluetoothEmulationSetSimulatedCentralStateParams {
    pub state: Box<CentralState>,
}
/// Set the state of the simulated central.
pub type BluetoothEmulationSetSimulatedCentralStateReturns = ();
/// Disable the BluetoothEmulation domain.
pub type BluetoothEmulationDisableParams = ();
/// Disable the BluetoothEmulation domain.
pub type BluetoothEmulationDisableReturns = ();
/** Simulates a peripheral with |address|, |name| and |knownServiceUuids|
that has already been connected to the system.*/
pub struct BluetoothEmulationSimulatePreconnectedPeripheralParams {
    pub address: String,
    pub name: String,
    pub manufacturer_data: Vec<ManufacturerData>,
    pub known_service_uuids: Vec<String>,
}
/** Simulates a peripheral with |address|, |name| and |knownServiceUuids|
that has already been connected to the system.*/
pub type BluetoothEmulationSimulatePreconnectedPeripheralReturns = ();
/** Simulates an advertisement packet described in |entry| being received by
the central.*/
pub struct BluetoothEmulationSimulateAdvertisementParams {
    pub entry: Box<ScanEntry>,
}
/** Simulates an advertisement packet described in |entry| being received by
the central.*/
pub type BluetoothEmulationSimulateAdvertisementReturns = ();
/** Simulates the response code from the peripheral with |address| for a
GATT operation of |type|. The |code| value follows the HCI Error Codes from
Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.*/
pub struct BluetoothEmulationSimulateGattOperationResponseParams {
    pub address: String,
    pub _type: Box<GattOperationType>,
    pub code: i64,
}
/** Simulates the response code from the peripheral with |address| for a
GATT operation of |type|. The |code| value follows the HCI Error Codes from
Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.*/
pub type BluetoothEmulationSimulateGattOperationResponseReturns = ();
/** Simulates the response from the characteristic with |characteristicId| for a
characteristic operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub struct BluetoothEmulationSimulateCharacteristicOperationResponseParams {
    pub characteristic_id: String,
    pub _type: Box<CharacteristicOperationType>,
    pub code: i64,
    pub data: String,
}
/** Simulates the response from the characteristic with |characteristicId| for a
characteristic operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub type BluetoothEmulationSimulateCharacteristicOperationResponseReturns = ();
/** Simulates the response from the descriptor with |descriptorId| for a
descriptor operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub struct BluetoothEmulationSimulateDescriptorOperationResponseParams {
    pub descriptor_id: String,
    pub _type: Box<DescriptorOperationType>,
    pub code: i64,
    pub data: String,
}
/** Simulates the response from the descriptor with |descriptorId| for a
descriptor operation of |type|. The |code| value follows the Error
Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
The |data| is expected to exist when simulating a successful read operation
response.*/
pub type BluetoothEmulationSimulateDescriptorOperationResponseReturns = ();
/// Adds a service with |serviceUuid| to the peripheral with |address|.
pub struct BluetoothEmulationAddServiceParams {
    pub address: String,
    pub service_uuid: String,
}
/// Adds a service with |serviceUuid| to the peripheral with |address|.
pub struct BluetoothEmulationAddServiceParams {
    pub service_id: String,
}
/// Removes the service respresented by |serviceId| from the simulated central.
pub struct BluetoothEmulationRemoveServiceParams {
    pub service_id: String,
}
/// Removes the service respresented by |serviceId| from the simulated central.
pub type BluetoothEmulationRemoveServiceReturns = ();
/** Adds a characteristic with |characteristicUuid| and |properties| to the
service represented by |serviceId|.*/
pub struct BluetoothEmulationAddCharacteristicParams {
    pub service_id: String,
    pub characteristic_uuid: String,
    pub properties: Box<CharacteristicProperties>,
}
/** Adds a characteristic with |characteristicUuid| and |properties| to the
service represented by |serviceId|.*/
pub struct BluetoothEmulationAddCharacteristicParams {
    pub characteristic_id: String,
}
/** Removes the characteristic respresented by |characteristicId| from the
simulated central.*/
pub struct BluetoothEmulationRemoveCharacteristicParams {
    pub characteristic_id: String,
}
/** Removes the characteristic respresented by |characteristicId| from the
simulated central.*/
pub type BluetoothEmulationRemoveCharacteristicReturns = ();
/** Adds a descriptor with |descriptorUuid| to the characteristic respresented
by |characteristicId|.*/
pub struct BluetoothEmulationAddDescriptorParams {
    pub characteristic_id: String,
    pub descriptor_uuid: String,
}
/** Adds a descriptor with |descriptorUuid| to the characteristic respresented
by |characteristicId|.*/
pub struct BluetoothEmulationAddDescriptorParams {
    pub descriptor_id: String,
}
/// Removes the descriptor with |descriptorId| from the simulated central.
pub struct BluetoothEmulationRemoveDescriptorParams {
    pub descriptor_id: String,
}
/// Removes the descriptor with |descriptorId| from the simulated central.
pub type BluetoothEmulationRemoveDescriptorReturns = ();
/// Simulates a GATT disconnection from the peripheral with |address|.
pub struct BluetoothEmulationSimulateGattDisconnectionParams {
    pub address: String,
}
/// Simulates a GATT disconnection from the peripheral with |address|.
pub type BluetoothEmulationSimulateGattDisconnectionReturns = ();
/** Event for when a GATT operation of |type| to the peripheral with |address|
happened.*/
pub struct BluetoothEmulationGattOperationReceivedEvent {
    pub address: String,
    pub _type: Box<GattOperationType>,
}
/** Event for when a characteristic operation of |type| to the characteristic
respresented by |characteristicId| happened. |data| and |writeType| is
expected to exist when |type| is write.*/
pub struct BluetoothEmulationCharacteristicOperationReceivedEvent {
    pub characteristic_id: String,
    pub _type: Box<CharacteristicOperationType>,
    pub data: String,
    pub write_type: Box<CharacteristicWriteType>,
}
/** Event for when a descriptor operation of |type| to the descriptor
respresented by |descriptorId| happened. |data| is expected to exist when
|type| is write.*/
pub struct BluetoothEmulationDescriptorOperationReceivedEvent {
    pub descriptor_id: String,
    pub _type: Box<CharacteristicOperationType>,
    pub data: String,
}
