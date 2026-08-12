plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
}

android {
    namespace = "ing.croft.call"
    compileSdk = 35

    defaultConfig {
        applicationId = "ing.croft.call"
        minSdk = 26            // matches iroh reference app floor (Android 8.0)
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions { jvmTarget = "17" }
    buildFeatures { compose = true }
}

dependencies {
    // iroh Kotlin bindings from Maven Central. Per n0's reference Android app,
    // this artifact bundles libiroh_ffi.so for every Android ABI (no NDK).
    // If your resolved version lacks Android ABIs (older docs said the artifact
    // was single-platform), fall back to building iroh-ffi from source; see README.
    implementation("computer.iroh:iroh:1.0.0") {
        // Quirk from the reference app: the artifact declares plain-jar JNA
        // transitively, but Android needs the @aar variant which bundles
        // libjnidispatch.so per ABI. Keeping both duplicates classes at packaging.
        exclude(group = "net.java.dev.jna", module = "jna")
    }
    implementation("net.java.dev.jna:jna:5.14.0@aar")  // uniffi requires JNA >= 5.12

    implementation("androidx.security:security-crypto:1.1.0-alpha06") // EncryptedSharedPreferences
    implementation("androidx.activity:activity-compose:1.9.3")
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.8.7")
    implementation(platform("androidx.compose:compose-bom:2024.12.01"))
    implementation("androidx.compose.material3:material3")
    implementation("androidx.compose.ui:ui")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.9.0")
}
