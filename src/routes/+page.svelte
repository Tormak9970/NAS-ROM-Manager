<script lang="ts">
  import jsSHA from "jssha";
  import { Card } from "@layout";
  import { TextField, Button } from "@interactables";
  import { LogController, RustInterop } from "@controllers";
  import { VisibiityOff, Visibility } from "@icons";
  import { showErrorSnackbar } from "@stores/State";

  let user = $state("");
  let password = $state("");
  let passwordVisible = $state(false);

  async function signIn() {
    const shaObj = new jsSHA("SHA-256", "TEXT", { encoding: "UTF8" });
    shaObj.update(password);

    const hash = shaObj.getHash("HEX");
    const result = await RustInterop.authenticate(user, hash);

    LogController.log("Authenticated:", result);

    if (!result) {
      $showErrorSnackbar({
        message: "Login Failed. Please Try Again"
      });
    }
  }
</script>

<div class="login-form-wrapper">
  <div id="login-form">
    <Card type="elevated">
      <div class="body">
        <h2 class="header">Login</h2>
        <TextField
          name="Username"
          bind:value={user}
        />
        <TextField
          name="Password"
          trailingIcon={passwordVisible ? VisibiityOff : Visibility}
          extraOptions={{
            type: passwordVisible ? "text" : "password"
          }}
          on:trailingClick={() => passwordVisible = !passwordVisible}
          bind:value={password}
        />
        <Button
          type="tonal"
          extraOptions={{ style: "width: 100%; margin-top: 1rem;" }}
          on:click={signIn}
          disabled={user === "" || password === ""}
        >
          Sign In
        </Button>
      </div>
    </Card>
  </div>
  <div class="floater-text">NAS ROM Manager</div>
</div>

<style>
  .login-form-wrapper {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  #login-form {
    max-width: 20rem;

    --m3-util-background: var(--m3-scheme-surface-container-low);
  }

  .body {
    display: flex;
    flex-direction: column;
    align-items: center;
    
    gap: 0.75rem;
  }

  .header {
    margin: 1rem 0;
  }

  .floater-text {
    margin-top: 1rem;
    font-size: small;

    color: rgb(var(--m3-scheme-surface-container));
  }
</style>