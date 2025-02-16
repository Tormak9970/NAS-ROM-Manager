<script lang="ts">
  import { AuthController, LogController } from "@controllers";
  import { VisibiityOff, Visibility } from "@icons";
  import { Button, Checkbox, TextField } from "@interactables";
  import { Card } from "@layout";
  import { rememberMe } from "@stores/Auth";
  import { showWarningSnackbar } from "@stores/State";
  import jsSHA from "jssha";

  let user = $state("");
  let password = $state("");
  let passwordVisible = $state(false);

  async function signIn() {
    const shaObj = new jsSHA("SHA-256", "TEXT", { encoding: "UTF8" });
    shaObj.update(password);

    const hash = shaObj.getHash("HEX");
    const result = await AuthController.authenticate(user, hash);

    LogController.log("Authenticated:", result);

    if (!result) {
      $showWarningSnackbar({
        message: "Login Failed. Please Try Again"
      });
    }
  }
</script>

<svelte:head>
	<title>Login</title>
</svelte:head>

<div class="login-form-wrapper">
  <div id="login-form">
    <Card type="elevated">
      <div slot="header" class="card-header">
        <img src="/logo.svg" alt="Logo" />
      </div>
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
        <div class="remember-me-container">
          <Checkbox bind:checked={$rememberMe} />
          Remember me
        </div>
        <Button
          type="tonal"
          extraOptions={{ style: "width: 100%;" }}
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

    position: relative;
  }

  .card-header {
    width: 100%;
    height: 3.5rem;

    display: flex;
    justify-content: center;
    align-items: center;

    background-color: rgb(var(--m3-scheme-surface-container-high));
  }

  .card-header > img {
    width: 2.5rem;
    height: 2.5rem;
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
    margin: 0.5rem 0;
  }

  .remember-me-container {
    width: calc(100% - 0.5rem);

    display: flex;
    align-items: center;
    gap: 0.5rem;

    margin: 0.5rem 0.25rem;
  }

  .floater-text {
    margin-top: 1rem;
    font-size: small;

    color: rgb(var(--m3-scheme-surface-container));
  }
</style>