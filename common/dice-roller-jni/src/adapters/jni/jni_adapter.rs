use jni::objects::{JClass, JObject};
use jni::sys::jbyteArray;
use jni::JNIEnv;

#[cfg(feature = "enable-android-logging")]
use android_logger::Config;
#[cfg(feature = "enable-android-logging")]
use log::Level;
use dice_roller::roll::roll_request::RollRequest;
use dice_roller::roll::roll_dice::roll_dice;

#[cfg(feature = "enable-android-logging")]
fn initialize_android_logger() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace)
            .with_tag("lib-dice-roller-jni"),
    );

    trace!("Android logging enabled");
}

#[no_mangle]
pub extern "system" fn Java_com_walterscarborough_libdiceroller_DefaultDiceRollerRepository_roll_1dice(
    env: JNIEnv,
    _class: JClass,
    input_byte_array: jbyteArray,
) -> jbyteArray {
    #[cfg(feature = "enable-android-logging")]
    initialize_android_logger();

    let roll_request_byte_array = env
        .convert_byte_array(input_byte_array)
        .expect("should be able to decode input byte array");
    let roll_request = RollRequest::from_protobuf(roll_request_byte_array);

    let roll_response = roll_dice(roll_request);

    let serialized_roll_response = roll_response.to_protobuf();
    let output = env
        .byte_array_from_slice(serialized_roll_response.as_slice())
        .expect("should be able to convert rust byte array to jvm byte array");

    JObject::from(output).into_inner()
}
