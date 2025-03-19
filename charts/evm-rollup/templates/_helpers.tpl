{{/*
Namepsace to deploy elements into.
*/}}
{{- define "rollup.namespace" -}}
{{- default .Release.Namespace .Values.global.namespaceOverride | trunc 63 | trimSuffix "-" -}}
{{- end }}

{{/*  The name of the rollup */}}
{{- define "rollup.name" -}}
{{- tpl .Values.global.rollupName . }}
{{- end }}

{{- define "rollup.genesis-file" -}}
files/genesis/{{ include "rollup.type" . }}.genesis.json
{{- end -}}

{{- define "rollup.tags.geth" -}}
{{- $rollupType := (include "rollup.type" . ) -}}
{{- if or (eq $rollupType "custom") .Values.global.dev -}}{{ .Values.images.geth.tag }}
{{- else if eq $rollupType "flame-mainnet" -}}1.1.0
{{- else if eq $rollupType "flame-testnet" -}}1.1.0
{{- else if eq $rollupType "flame-devnet" -}}1.1.0
{{- end -}}
{{- end }}

{{- define "rollup.tags.conductor" -}}
{{- $rollupType := (include "rollup.type" . ) -}}
{{- if or (eq $rollupType "custom") .Values.global.dev -}}{{ .Values.images.conductor.tag }}
{{- else if eq $rollupType "flame-mainnet" -}}1.1.0
{{- else if eq $rollupType "flame-testnet" -}}1.1.0
{{- else if eq $rollupType "flame-devnet" -}}1.1.0
{{- end -}}
{{- end }}


{{- define "rollup.type" -}}
{{- $rollupName := (include "rollup.name" . ) -}}
{{- if eq $rollupName "flame" -}}flame-mainnet
{{- else if eq $rollupName "flame-dawn-1" -}}flame-testnet
{{- else if eq $rollupName "astria-dusk-11-nria-evm"}}flame-devnet
{{- else -}}custom
{{- end -}}
{{- end }}


{{/* verbosity based on log level */}}
{{- define "rollup.verbosity" -}}
{{- if eq . "silent" }}0
{{- else if eq . "error" }}1
{{- else if eq . "warn" }}2
{{- else if eq . "info" }}3
{{- else if eq . "debug" }}4
{{- else if eq . "trace" }}5
{{- end }}
{{- end }}

{{- define "rollup.moduleVerbosity" -}}
{{- range $module := .Values.geth.moduleLogLevels }}{{$module.module}}={{ include "rollup.verbosity" $module.level }},
{{- end }}
{{- end }}

{{/*
Expand the name of the chart.
*/}}
{{- define "rollup.appName" -}}
{{- default (include "rollup.name" .) | trunc 63 | trimSuffix "-" }}-astria-dev-cluster
{{- end }}

{{/*
Common labels
*/}}
{{- define "rollup.labels" -}}
{{ include "rollup.selectorLabels" . }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "rollup.selectorLabels" -}}
app: {{ include "rollup.appName" . }}
{{- end }}

{{/*
Full image paths for Astria built images
*/}}
{{- define "rollup.image" -}}
{{ .Values.images.geth.repo }}:{{ include "rollup.tags.geth" . }}
{{- end }}

{{- define "conductor.image" -}}
{{ .Values.images.conductor.repo }}:{{ include "rollup.tags.conductor" . }}
{{- end }}


{{/*
Return if ingress is stable.
*/}}
{{- define "rollup.ingress.isStable" -}}
{{- eq (include "rollup.ingress.apiVersion" .) "networking.k8s.io/v1" }}
{{- end }}

{{/*
Return if ingress supports ingressClassName.
*/}}
{{- define "rollup.ingress.supportsIngressClassName" -}}
{{- or (eq (include "rollup.ingress.isStable" .) "true") (and (eq (include "rollup.ingress.apiVersion" .) "networking.k8s.io/v1beta1") (semverCompare ">= 1.18-0" .Capabilities.KubeVersion.Version)) }}
{{- end }}

{{/*
Return if ingress supports pathType.
*/}}
{{- define "rollup.ingress.supportsPathType" -}}
{{- or (eq (include "rollup.ingress.isStable" .) "true") (and (eq (include "rollup.ingress.apiVersion" .) "networking.k8s.io/v1beta1") (semverCompare ">= 1.18-0" .Capabilities.KubeVersion.Version)) }}
{{- end }}

{{/*
Return the appropriate apiVersion for ingress.
*/}}
{{- define "rollup.ingress.apiVersion" -}}
{{- if and ($.Capabilities.APIVersions.Has "networking.k8s.io/v1") (semverCompare ">= 1.19-0" .Capabilities.KubeVersion.Version) }}
{{- print "networking.k8s.io/v1" }}
{{- else if $.Capabilities.APIVersions.Has "networking.k8s.io/v1beta1" }}
{{- print "networking.k8s.io/v1beta1" }}
{{- else }}
{{- print "extensions/v1beta1" }}
{{- end }}
{{- end }}

{{- define "rollup.gethHomeDir" -}}
/home/geth
{{- end }}

{{- define "rollup.gethDataDir" -}}
{{ include "rollup.gethHomeDir" . }}/{{ include "rollup.name" . }}
{{- end }}
