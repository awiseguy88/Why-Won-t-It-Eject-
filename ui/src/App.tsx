import { useCallback, useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type UiVolume = {
  id: string;
  label?: string | null;
  mount_point?: string | null;
  fs_type?: string | null;
};

type ScopeStatus =
  | "Eligible"
  | { Attempting: { operation_id: string } }
  | { Busy: { diagnostics?: unknown } }
  | { RetryArmed: { retry: { next_attempt_at_unix_ms?: number; deadline_at_unix_ms: number; attempt_count: number }; diagnostics?: unknown } }
  | { RetryExpired: { diagnostics?: unknown } }
  | "SafeToUnplug"
  | "DeviceGone"
  | { Partial: { detail: string } }
  | { PermissionDenied: { detail?: string | null } }
  | { Unsupported: { detail: string } }
  | { UnknownOutcome: { detail: string } }
  | { Failure: { detail: string } };

type UiDevice = {
  id: string;
  generation: number;
  display_name: string;
  model?: string | null;
  size_bytes?: number | null;
  eligible: boolean;
  ineligible_reason?: string | null;
  volumes: UiVolume[];
  status: ScopeStatus;
};

function formatBytes(bytes?: number | null): string {
  if (bytes == null) return "";
  const units = ["B", "KB", "MB", "GB", "TB"];
  let v = bytes;
  let i = 0;
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024;
    i++;
  }
  return `${v.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

function statusLabel(status: ScopeStatus): { tone: "ok" | "warn" | "err" | "info"; text: string; detail?: string } {
  if (status === "Eligible") return { tone: "info", text: "Ready to eject" };
  if (status === "SafeToUnplug") return { tone: "ok", text: "Safe to unplug" };
  if (status === "DeviceGone") return { tone: "info", text: "Drive disconnected" };
  if (typeof status === "object" && "Attempting" in status) return { tone: "info", text: "Ejecting…" };
  if (typeof status === "object" && "Busy" in status)
    return { tone: "warn", text: "Still in use", detail: "Close files on this drive, then try again." };
  if (typeof status === "object" && "RetryArmed" in status)
    return { tone: "warn", text: "Waiting to eject when ready", detail: "Automatic retry is armed." };
  if (typeof status === "object" && "RetryExpired" in status)
    return { tone: "warn", text: "Automatic retry stopped", detail: "Offer a fresh manual attempt if eligible." };
  if (typeof status === "object" && "Partial" in status)
    return { tone: "warn", text: "Partially ejected", detail: status.Partial.detail };
  if (typeof status === "object" && "PermissionDenied" in status)
    return { tone: "err", text: "Permission denied", detail: status.PermissionDenied.detail ?? undefined };
  if (typeof status === "object" && "Unsupported" in status)
    return { tone: "err", text: "Unsupported", detail: status.Unsupported.detail };
  if (typeof status === "object" && "UnknownOutcome" in status)
    return { tone: "err", text: "Unknown outcome", detail: status.UnknownOutcome.detail };
  if (typeof status === "object" && "Failure" in status)
    return { tone: "err", text: "Eject failed", detail: status.Failure.detail };
  return { tone: "info", text: "Unknown" };
}

function toneClasses(tone: "ok" | "warn" | "err" | "info") {
  switch (tone) {
    case "ok":
      return "border-green-300 bg-green-50 text-green-900 dark:border-green-800 dark:bg-green-950 dark:text-green-100";
    case "warn":
      return "border-amber-300 bg-amber-50 text-amber-950 dark:border-amber-800 dark:bg-amber-950 dark:text-amber-100";
    case "err":
      return "border-red-300 bg-red-50 text-red-950 dark:border-red-800 dark:bg-red-950 dark:text-red-100";
    default:
      return "border-blue-200 bg-blue-50 text-blue-950 dark:border-blue-900 dark:bg-blue-950 dark:text-blue-100";
  }
}

export default function App() {
  const [devices, setDevices] = useState<UiDevice[]>([]);
  const [selectedKey, setSelectedKey] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const refresh = useCallback(async () => {
    try {
      setLoading(true);
      const list = await invoke<UiDevice[]>("list_devices");
      setDevices(list);
      setError(null);
      // Start with no selection when multiple devices are present.
      if (list.length === 1) setSelectedKey(`${list[0].id}::${list[0].generation}`);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const selected = useMemo(
    () => devices.find((d) => `${d.id}::${d.generation}` === selectedKey) ?? null,
    [devices, selectedKey]
  );

  const doEject = async (d: UiDevice) => {
    try {
      setError(null);
      const status = await invoke<ScopeStatus>("request_eject", { deviceId: d.id, generation: d.generation });
      setDevices((prev) => prev.map((x) => (x.id === d.id && x.generation === d.generation ? { ...x, status } : x)));
    } catch (e) {
      setError(String(e));
    }
  };

  const doArmRetry = async (d: UiDevice) => {
    try {
      setError(null);
      const status = await invoke<ScopeStatus>("arm_retry", { deviceId: d.id, generation: d.generation });
      setDevices((prev) => prev.map((x) => (x.id === d.id && x.generation === d.generation ? { ...x, status } : x)));
    } catch (e) {
      setError(String(e));
    }
  };

  const doCancelRetry = async (d: UiDevice) => {
    try {
      setError(null);
      const status = await invoke<ScopeStatus>("cancel_retry", { deviceId: d.id, generation: d.generation });
      setDevices((prev) => prev.map((x) => (x.id === d.id && x.generation === d.generation ? { ...x, status } : x)));
    } catch (e) {
      setError(String(e));
    }
  };

  const isBusy = (s: ScopeStatus) => typeof s === "object" && "Busy" in s;
  const isArmed = (s: ScopeStatus) => typeof s === "object" && "RetryArmed" in s;

  return (
    <main className="min-h-screen p-6">
      <header className="mb-6 flex items-start justify-between gap-4">
        <div>
          <h1 className="text-2xl font-semibold tracking-tight">Why Won't It Eject?</h1>
          <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
            Select a drive, request safe eject, and see what your computer can identify if it’s busy.
          </p>
        </div>
        <button
          onClick={() => void refresh()}
          className="rounded-md border border-gray-300 dark:border-gray-700 px-3 py-2 text-sm hover:bg-gray-50 dark:hover:bg-gray-900"
          aria-label="Refresh device list"
        >
          {loading ? "Refreshing…" : "Refresh"}
        </button>
      </header>

      {error && (
        <div role="alert" className={`mb-4 rounded-md border p-3 text-sm ${toneClasses("err")}`}>
          {error}
        </div>
      )}

      <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
        <section aria-label="Removable devices">
          <h2 className="mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">Drives</h2>
          {devices.length === 0 ? (
            <div className="status-card text-sm text-gray-600 dark:text-gray-400">
              No removable USB drives detected. Connect a drive to get started.
            </div>
          ) : (
            <ul className="space-y-2">
              {devices.map((d) => {
                const key = `${d.id}::${d.generation}`;
                const active = key === selectedKey;
                return (
                  <li key={key}>
                    <button
                      onClick={() => setSelectedKey(key)}
                      className={`w-full rounded-lg border p-3 text-left transition ${
                        active
                          ? "border-brand-600 bg-brand-50 dark:bg-blue-950/30"
                          : "border-gray-200 hover:border-gray-300 dark:border-gray-800 dark:hover:border-gray-700"
                      }`}
                      aria-pressed={active}
                    >
                      <div className="flex items-center justify-between gap-2">
                        <span className="font-medium">{d.display_name}</span>
                        <span className="text-xs text-gray-500">{formatBytes(d.size_bytes)}</span>
                      </div>
                      <div className="mt-1 text-xs text-gray-600 dark:text-gray-400">
                        {d.volumes.length} volume{d.volumes.length === 1 ? "" : "s"}
                        {d.volumes[0]?.mount_point ? ` · ${d.volumes[0].mount_point}` : ""}
                      </div>
                      {!d.eligible && d.ineligible_reason && (
                        <div className="mt-1 text-xs text-amber-700 dark:text-amber-400">{d.ineligible_reason}</div>
                      )}
                    </button>
                  </li>
                );
              })}
            </ul>
          )}
        </section>

        <section aria-label="Selected drive status">
          <h2 className="mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">Status</h2>
          {!selected ? (
            <div className="status-card text-sm text-gray-600 dark:text-gray-400">
              {devices.length > 1 ? "Select a physical device to continue." : "Select a drive to see details."}
            </div>
          ) : (
            <div className="space-y-4">
              <div className={`rounded-lg border p-4 ${toneClasses(statusLabel(selected.status).tone)}`} aria-live="polite">
                <div className="font-medium">{statusLabel(selected.status).text}</div>
                {statusLabel(selected.status).detail && (
                  <p className="mt-1 text-sm opacity-90">{statusLabel(selected.status).detail}</p>
                )}
              </div>

              <div className="status-card">
                <h3 className="text-sm font-medium">Volumes & mount locations</h3>
                {selected.volumes.length === 0 ? (
                  <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">No mounted volumes reported.</p>
                ) : (
                  <ul className="mt-2 space-y-2 text-sm">
                    {selected.volumes.map((v) => (
                      <li key={v.id} className="flex flex-col">
                        <span className="font-medium">{v.label || v.id}</span>
                        <span className="text-gray-600 dark:text-gray-400">
                          {v.mount_point || "(not mounted)"} {v.fs_type ? `· ${v.fs_type}` : ""}
                        </span>
                      </li>
                    ))}
                  </ul>
                )}
              </div>

              <div className="flex flex-wrap gap-2">
                <button
                  onClick={() => void doEject(selected)}
                  disabled={!selected.eligible || typeof selected.status === "object" && "Attempting" in selected.status}
                  className="rounded-md bg-brand-600 px-4 py-2 text-sm font-medium text-white hover:bg-brand-700 disabled:cursor-not-allowed disabled:opacity-50"
                >
                  {isBusy(selected.status) ? "Try again" : "Safely eject"}
                </button>
                {isBusy(selected.status) && (
                  <button
                    onClick={() => void doArmRetry(selected)}
                    className="rounded-md border border-gray-300 dark:border-gray-700 px-4 py-2 text-sm hover:bg-gray-50 dark:hover:bg-gray-900"
                  >
                    Eject when ready
                  </button>
                )}
                {isArmed(selected.status) && (
                  <button
                    onClick={() => void doCancelRetry(selected)}
                    className="rounded-md border border-gray-300 dark:border-gray-700 px-4 py-2 text-sm hover:bg-gray-50 dark:hover:bg-gray-900"
                  >
                    Cancel retry
                  </button>
                )}
              </div>

              <p className="text-xs text-gray-500 dark:text-gray-500">
                Eject affects all volumes shown for this physical drive. If multiple volumes or sibling drives are involved,
                the app will name each one before proceeding (backend scope confirmation to be wired in native adapters).
              </p>
            </div>
          )}
        </section>
      </div>
    </main>
  );
}